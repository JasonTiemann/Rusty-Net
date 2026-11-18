use crate::brain::neuron::base_neuron::BaseNeuron;
use crate::brain::neuron::dendrite::Dendrite;
use crate::constants::{
    BASELINE_REGRESSION_SPEED, NEURON_HISTORY_LENGTH, POTENTIAL_DENDRITE_ACCEPT_THRESHOLD, TOTAL_NEURONS, TRIGGER_THRESHOLD
};

pub struct LearningNeuron{
    pub dendrites: Vec<Dendrite>,
    pub potential_dendrites: Vec<Dendrite>,
    pub output_history: [usize; NEURON_HISTORY_LENGTH],
    pub value: f32,
}

impl LearningNeuron{
    pub fn new(dendrite_targets: &Vec<usize>) -> Self{
        let dendrites = dendrite_targets.iter().map(|dt| Dendrite::new(*dt)).collect();
        Self{
            dendrites,
            potential_dendrites: vec![],
            output_history: [0; NEURON_HISTORY_LENGTH],
            value: 0.0,
        }
    }


    pub fn activate(&mut self, prior_activations: &[bool;TOTAL_NEURONS], dopamine: f32) -> bool {
        // Gather inputs
        for dendrite in &self.dendrites{
            self.value += dendrite.get_output(prior_activations);
        }
        let active = self.value >= TRIGGER_THRESHOLD;


        // Downgrade weak dendrites
        let downgraded_dendrites: Vec<Dendrite> = self.dendrites.extract_if(.., |d| {
            d.adjust_strength(prior_activations, dopamine);
            d.strength < POTENTIAL_DENDRITE_ACCEPT_THRESHOLD
        }).collect();

        // Upgrade strong dendrites
        let upgraded_dendrites: Vec<Dendrite> = self.potential_dendrites.extract_if(.., |d| {
            d.adjust_strength(prior_activations, dopamine);
            d.strength > POTENTIAL_DENDRITE_ACCEPT_THRESHOLD
        }).collect();

        // Remove failed dendrites
        self.potential_dendrites.retain(|d| d.strength > 0.0);

        // Place moved dendrites
        self.dendrites.extend(upgraded_dendrites);
        self.potential_dendrites.extend(downgraded_dendrites);
        
        
        // Determine output and regress to mean
        if active { self.value = 0.0; }
        self.value *= BASELINE_REGRESSION_SPEED;

        active
    }
}


impl From<BaseNeuron> for LearningNeuron{
    fn from(n: BaseNeuron) -> Self{
        let mut dendrites = n.dendrites;
        let potential_dendrites = dendrites.extract_if(.., |d| d.strength < 0.15).collect();

        Self { 
            dendrites, 
            potential_dendrites, 
            output_history: [0; NEURON_HISTORY_LENGTH],
            value: 0.0,
        }
    }
}