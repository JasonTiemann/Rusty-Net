use crate::constants::{INPUT_NEURON_BASE_THRESHOLD, INPUT_NEURONS, NEURON_HISTORY_LENGTH};

pub struct InputNeuron{
    threshold: f32,
    activation_history: [bool; NEURON_HISTORY_LENGTH],
    position: usize,
}

impl InputNeuron{
    pub fn new(position: usize) -> Self{
        Self{
            threshold: INPUT_NEURON_BASE_THRESHOLD,
            activation_history: [false; NEURON_HISTORY_LENGTH],
            position,
        }
    }

    pub fn activate(&self, input_values: [f32; INPUT_NEURONS]) -> bool {
        let activated = input_values[self.position] >= self.threshold;
        // self.activation_history.rotate_right(1);
        // self.activation_history[0] = activated;
        activated
    }
}