use crate::constants::{
    DENDRITE_STRENGTHEN_FACTOR,
    DENDRITE_GEN_STRENGTH, 
    DENDRITE_MAX_STRENGTH, 
};


pub struct Dendrite{
    pub target: usize,
    pub strength: f32,
}

impl Dendrite{
    pub fn new(target: usize) -> Self {
        Self{
            target,
            strength: DENDRITE_GEN_STRENGTH
        }
    }

    pub fn adjust_strength(&mut self, active_neurons: &Vec<usize>, dopamine: f32){
        self.strength += DENDRITE_STRENGTHEN_FACTOR 
                * dopamine
                * (if active_neurons.contains(&self.target) {1.0} else {-1.0});

        if (self.strength > DENDRITE_MAX_STRENGTH){
            self.strength = DENDRITE_MAX_STRENGTH;
        }
    }
}