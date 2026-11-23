mod dendrite;
mod learning_neuron;
mod input_neuron;
mod base_neuron;

use base_neuron::BaseNeuron;
use learning_neuron::LearningNeuron;
use input_neuron::InputNeuron;
use dendrite::Dendrite;

use crate::constants::{
	INPUT_NEURONS, TOTAL_NEURONS,
};


pub enum Neuron{
    Base(BaseNeuron),
    Input(InputNeuron),
    Learning(LearningNeuron),
}

impl Neuron{
    pub fn new_input(position: usize) -> Self{
        Neuron::Input(InputNeuron::new(position))
    }
    pub fn new_learning(dendrite_targets: &Vec<usize>) -> Self{
        Neuron::Learning(LearningNeuron::new(dendrite_targets))
    }
    pub fn new_base(dendrite_targets: &Vec<usize>) -> Self{
        Neuron::Base(BaseNeuron::new(dendrite_targets))
    }

    pub fn to_learning(self) -> Neuron{
        match self{
            Neuron::Input(_n) => Neuron::Learning(LearningNeuron::new(&vec![])),
            Neuron::Base(n) => Neuron::Learning(LearningNeuron::from_base(n)),
            Neuron::Learning(n) => Neuron::Learning(n),
        }
    }

    pub fn activate(&mut self, input_values: [f32; INPUT_NEURONS], prior_activations: &[bool;TOTAL_NEURONS], dopamine: f32) -> bool{
        match self{
            Neuron::Input(n) => n.activate(input_values),
            Neuron::Base(n) => n.activate(prior_activations),
            Neuron::Learning(n) => n.activate(prior_activations, dopamine)
        }
    }
}