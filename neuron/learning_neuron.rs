pub struct LearningNeuron{
    dendrites: Vec<Dendrite>,
    potential_dendrites: Vec<Dendrite>,
    output_history: [usize; NEURON_HISTORY_LENGTH],
    pub value: f32,
}

