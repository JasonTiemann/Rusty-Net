use crate::constants::{DENDRITE_STRENGTH_FACTOR, DENDRITE_GEN_STRENGTH, DENDRITE_MAX_STRENGTH};


pub struct Dendrite{
    pub target: i64,
    pub strength: f32,
}

impl Dendrite{
    pub fn new(target: i64) -> Self {
        Self{
            target,
            strength: DENDRITE_GEN_STRENGTH,
        }
    }
}


pub struct Neuron{
    pub dendrites: Vec<Dendrite>,
}