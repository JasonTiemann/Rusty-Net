// Holds all the neurons, controls the tick speed and dopamine
mod neuron;
use neuron::Neuron;

pub struct Brain{
    neurons: Vec<Neuron>,
    input_size: 
    isLearning: bool,
    currentValue: f32,
}

impl Brain{
    pub fn new(&mut self, neuron_count: usize, input_size: usize, output_size: usize, is_learning: bool) -> Self{
        let neurons: Vec<Neuron> = vec![];

        for i in 0..neuron_count{
            if i < input_size{
                
            }
        }

        Self{
        }
    }
}