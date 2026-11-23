// Holds all the neurons, controls the tick speed and dopamine
mod dopamine;
mod neuron;

use crate::brain::neuron::Neuron;
use crate::constants::{
    INPUT_NEURONS, OUTPUT_NEURONS, OUTPUT_SAMPLE, STARTING_DENDRITE_COUNT, TOTAL_NEURONS,
};
use dopamine::Dopamine;
use rand::seq::IteratorRandom;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

#[derive(Clone)]
pub struct BrainControl {
    inputs: Arc<Mutex<[f32; INPUT_NEURONS]>>,
    outputs: Arc<Mutex<[f32; OUTPUT_NEURONS]>>,
    dopamine: Dopamine,
}
impl BrainControl {
    pub fn set_inputs(&self, v: [f32; INPUT_NEURONS]) {
        *self.inputs.lock().unwrap() = v;
    }
    pub fn set_input(&self, i: usize, v: f32) {
        self.inputs.lock().unwrap()[i] = v;
    }
    pub fn get_inputs(&self) -> [f32; INPUT_NEURONS] {
        *self.inputs.lock().unwrap()
    }

    pub fn set_outputs(&self, v: [f32; OUTPUT_NEURONS]) {
        *self.outputs.lock().unwrap() = v;
    }
    pub fn get_outputs(&self) -> [f32; OUTPUT_NEURONS] {
        *self.outputs.lock().unwrap()
    }

    pub fn nudge_input(&self, i: usize, d: f32) {
        let mut g = self.inputs.lock().unwrap();
        g[i] = (g[i] + d).clamp(0.0, 1.0);
    }

    pub fn dopamine_up(&self) {
        self.dopamine.up();
    }
    pub fn dopamine_down(&self) {
        self.dopamine.down();
    }
    pub fn dopamine_get(&self) -> f32 {
        self.dopamine.get()
    }
    pub fn step_dopamine(&self) -> f32 {
        self.dopamine.step()
    }
}

pub struct Brain {
    neurons: Option<[Neuron; TOTAL_NEURONS]>,
    control: BrainControl,
    is_running: Arc<AtomicBool>,
    handle: Option<JoinHandle<[Neuron; TOTAL_NEURONS]>>,
}

impl Brain {
    pub fn new(is_learning: bool) -> Self {
        let mut rng = rand::rng();
        let neurons: [Neuron; TOTAL_NEURONS] = std::array::from_fn(|i| {
            if i < INPUT_NEURONS {
                Neuron::new_input(i)
            } else {
                let targets = (0..i).sample(&mut rng, STARTING_DENDRITE_COUNT);

                if is_learning {
                    Neuron::new_learning(&targets)
                } else {
                    Neuron::new_base(&targets)
                }
            }
        });

        Self {
            neurons: Some(neurons),
            is_running: Arc::new(AtomicBool::new(false)),
            handle: None,
            control: BrainControl {
                inputs: Arc::new(Mutex::new([0.0; INPUT_NEURONS])),
                outputs: Arc::new(Mutex::new([0.0; OUTPUT_NEURONS])),
                dopamine: Dopamine::new(),
            },
        }
    }

    pub fn start(&mut self) {
        let Some(mut neurons) = self.neurons.take() else {
            return;
        };

        self.is_running.store(true, Ordering::Release);

        let running = Arc::clone(&self.is_running);
        let mut local_control = self.control.clone();

        self.handle = Some(thread::spawn(move || {
            let mut output_history = [[false; OUTPUT_NEURONS]; OUTPUT_SAMPLE];
            let mut last_run = [false; TOTAL_NEURONS];

            while running.load(Ordering::Acquire) {
                let input_snapshot = local_control.get_inputs();
                let dope = local_control.step_dopamine();
                let current_run = neurons
                    .each_mut()
                    .map(|n| n.activate(input_snapshot, &last_run, dope));

                if let Some(&output) = current_run.last_chunk::<OUTPUT_NEURONS>() {
                    output_history[0] = output;
                }

                let mut numeric_output = [0.0; OUTPUT_NEURONS];
                for output in output_history {
                    for i in 0..OUTPUT_NEURONS {
                        if output[i] {
                            numeric_output[i] += 1.0 / (OUTPUT_NEURONS * OUTPUT_SAMPLE) as f32;
                        }
                    }
                }

                local_control.set_outputs(numeric_output);

                last_run = current_run;
            }

            neurons
        }));
    }

    pub fn get_controller(&self) -> BrainControl {
        self.control.clone()
    }

    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::Release);
        if let Some(handle) = self.handle.take() {
            self.neurons = Some(handle.join().unwrap());
        }
    }
}

impl Drop for Brain {
    fn drop(&mut self) {
        self.stop();
    }
}
