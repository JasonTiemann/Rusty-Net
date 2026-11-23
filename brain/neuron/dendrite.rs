use crate::constants::{
    DENDRITE_CORRELATION_DECAY, DENDRITE_GEN_STRENGTH, DENDRITE_MAX_STRENGTH,
    DENDRITE_STRENGTHEN_FACTOR, TOTAL_NEURONS,
};

pub struct Dendrite {
    pub target: usize,
    pub strength: f32,
    pub correlation: Option<f32>,
}

impl Dendrite {
    pub fn new(target: usize, is_learning: bool) -> Self {
        Self {
            target,
            strength: DENDRITE_GEN_STRENGTH,
            correlation: if is_learning { Some(0.0) } else { None },
        }
    }

    pub fn adjust_correlation(&mut self, last_fire: &[bool; TOTAL_NEURONS], self_fire: bool) {
        if let Some(correlation) = &mut self.correlation {
            *correlation *= 1.0 - DENDRITE_CORRELATION_DECAY;
            *correlation += (if last_fire[self.target] == self_fire {1.0} else {-1.0}) * DENDRITE_STRENGTHEN_FACTOR;
            *correlation = (*correlation).clamp(-1.0, 1.0);
        }
    }

    pub fn dopamine_adjust(&mut self, dopamine: f32) {
        if let Some(correlation) = self.correlation {
            self.strength += dopamine * correlation * DENDRITE_STRENGTHEN_FACTOR;
            self.strength = self.strength.clamp(-DENDRITE_MAX_STRENGTH, DENDRITE_MAX_STRENGTH);
        }
    }

    pub fn get_output(&self, last_fire: &[bool; TOTAL_NEURONS]) -> f32 {
        if last_fire[self.target] {
            return self.strength;
        }

        0.0
    }
}
