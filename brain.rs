// Holds all the neurons, controls the tick speed and dopamine
mod neuron;
use neuron::Neuron;

pub struct Brain{
    pub neurons: Vec<Neuron>,
    isLearning: bool,
    currentValue: f32,
}

impl Brain{

}