use crate::constants::{
    DENDRITE_GEN_STRENGTH, DENDRITE_MAX_STRENGTH, DENDRITE_STRENGTHEN_FACTOR, TOTAL_NEURONS, 
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

    pub fn adjust_strength(&mut self, active_neurons: &[bool;TOTAL_NEURONS], dopamine: f32){
        let adjustment = DENDRITE_STRENGTHEN_FACTOR * dopamine * (if active_neurons[self.target] {1.0} else {-1.0});

        self.strength = (self.strength + adjustment).min(DENDRITE_MAX_STRENGTH);
    }

    pub fn get_output(&self, active_neurons: &[bool;TOTAL_NEURONS]) -> f32{
        if active_neurons[self.target]{
            return self.strength;
        }

        0.0
    }
}