use crate::brain::neuron::Dendrite;
use crate::constants::{BASELINE_REGRESSION_SPEED, TOTAL_NEURONS, TRIGGER_THRESHOLD};

pub struct BaseNeuron{
    pub dendrites: Vec<Dendrite>,
    pub value: f32
}


impl BaseNeuron{
    pub fn new(dendrite_targets: &Vec<usize>) -> Self{
        let dendrites = dendrite_targets.iter().map(|dt| Dendrite::new(*dt)).collect();

        Self{
            dendrites,
            value: 0.0,
        }
    }


    pub fn activate(&mut self, prior_activations: &[bool; TOTAL_NEURONS]) -> bool {
        // Gather inputs
        for dendrite in &self.dendrites{
            self.value += dendrite.get_output(prior_activations);
        }
        
        // Determine output and regress to mean
        let active = self.value >= TRIGGER_THRESHOLD;

        if active{
            self.value = 0.0;
        }
        self.value *= BASELINE_REGRESSION_SPEED;

        active
    }
}

