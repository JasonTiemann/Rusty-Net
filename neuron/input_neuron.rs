pub struct InputNeuron{
    threshold: f32,
    activationHistory: [bool; NEURON_HISTORY_LENGTH],
}

impl InputNeuron{
    pub fn new() -> Self{
        Self{
            threshold: INPUT_NEURON_BASE_THRESHOLD,
            activationHistory: [false; NEURON_HISTORY_LENGTH],
        }
    }

    pub fn activate(&mut self, input_value: f32) -> bool {
        let activated = input_value >= self.threshold;
        self.activationHistory.rotate_right(1);
        self.activationHistory[0] = activated;
        activated
    }
}