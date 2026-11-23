// Holds all the neurons, controls the tick speed and dopamine
mod dopamine;
mod neuron;

use crate::brain::neuron::Neuron;
use crate::constants::{INPUT_NEURONS, OUTPUT_NEURONS, OUTPUT_SAMPLE, TOTAL_NEURONS};
use atomic_float::AtomicF32;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub struct Brain {
    neurons: Option<[Neuron; TOTAL_NEURONS]>,
    inputs: Arc<Mutex<[f32; INPUT_NEURONS]>>,
    dopamine: Arc<AtomicF32>,
    is_running: Arc<AtomicBool>,
    handle: Option<JoinHandle<[Neuron; TOTAL_NEURONS]>>,
}

impl Brain {
    pub fn new(is_learning: bool) -> Self {
        let neurons: [Neuron; TOTAL_NEURONS] = std::array::from_fn(|i| {
            if i < INPUT_NEURONS {
                Neuron::new_input(i)
            } else {
                let targets = (0..i).collect();

                if is_learning {
                    Neuron::new_learning(&targets)
                } else {
                    Neuron::new_base(&targets)
                }
            }
        });

        Self {
            neurons: Some(neurons),
            inputs: Arc::new(Mutex::new([0.0; INPUT_NEURONS])),
            dopamine: Arc::new(AtomicF32::new(1.0)),
            is_running: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    pub fn start<F>(&mut self, mut output_fn: F)
    where
        F: FnMut([f32; OUTPUT_NEURONS]) + Send + 'static,
    {
        let Some(mut neurons) = self.neurons.take() else {
            return;
        };
        self.is_running.store(true, Ordering::Release);

        let running = Arc::clone(&self.is_running);
        let dopamine = Arc::clone(&self.dopamine);
        let inputs = Arc::clone(&self.inputs);

        self.handle = Some(thread::spawn(move || {
            let mut output_history = [[false; OUTPUT_NEURONS]; OUTPUT_SAMPLE];
            let mut last_run = [false; TOTAL_NEURONS];

            let mut count: i64 = 0;

            while running.load(Ordering::Acquire) {
                let current_run = neurons.each_mut().map(|n| {
                    n.activate(
                        *inputs.lock().unwrap(),
                        &last_run,
                        dopamine.load(Ordering::Relaxed),
                    )
                });

                output_history.rotate_right(1);
                if let Some(&output) = current_run.last_chunk::<OUTPUT_NEURONS>() {
                    output_history[0] = output;
                }

                let mut numeric_output = [0.0; OUTPUT_NEURONS];
                for output in output_history {
                    for i in 0..OUTPUT_NEURONS {
                        numeric_output[i] += 1.0 / (OUTPUT_NEURONS as f32);
                    }
                }

                output_fn(numeric_output);

                last_run = current_run;
            }

            neurons
        }));
    }

    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::Release);
        if let Some(handle) = self.handle.take() {
            self.neurons = Some(handle.join().unwrap());
        }
    }

    pub fn update_inputs(&mut self, input_values: [f32; INPUT_NEURONS]) {
        let mut inputs = self.inputs.lock().unwrap();
        *inputs = input_values;
    }

    pub fn update_dopamine(&mut self, dopamine_value: f32) {
        self.dopamine.store(dopamine_value, Ordering::Relaxed);
    }
}

impl Drop for Brain {
    fn drop(&mut self) {
        self.stop();
    }
}
