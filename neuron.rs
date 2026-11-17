mod regular_neuron;
mod learning_neuron;
mod input_neuron;
mod dendrite;


use crate::constants::{
    TRIGGER_THRESHOLD,
    OUTPUT_AMT,
    DENDRITE_STRENGTHEN_FACTOR,
    DENDRITE_GEN_STRENGTH, 
    DENDRITE_MAX_STRENGTH, 
    POTENTIAL_DENDRITE_ACCEPT_THRESHOLD, 
    DOPAMINE_MULTI, 
    BASELINE_RETURN_SPEED,
    NEURON_HISTORY_LENGTH,
    INPUT_NEURON_BASE_THRESHOLD
};

pub enum Neuron{
    Regular(RegularNeuron),
    Input(InputNeuron),
    Learning(LearningNeuron),
}