use crate::constants::{
    TRIGGER_THRESHOLD,
    OUTPUT_AMT,
    DENDRITE_STRENGTHEN_FACTOR,
    DENDRITE_GEN_STRENGTH, 
    DENDRITE_MAX_STRENGTH, 
    POTENTIAL_DENDRITE_ACCEPT_THRESHOLD, 
    DOPAMINE_MULTI, 
    BASELINE_RETURN_SPEED,
};


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

    pub fn adjustStrength(&mut self, activeNeurons: &Vec<i64>, dopamine: bool){
        self.strength += DENDRITE_STRENGTHEN_FACTOR 
                * (if dopamine {DOPAMINE_MULTI} else {1.0}) 
                * (if activeNeurons.contains(&self.target) {1.0} else {-1.0});

        if (self.strength > DENDRITE_MAX_STRENGTH){
            self.strength = DENDRITE_MAX_STRENGTH;
        }
    }
}


pub struct Neuron{
    pub dendrites: Vec<Dendrite>,
    pub potentialDendrites: Vec<Dendrite>,
    pub value: f32,
}

impl Neuron{
    pub fn new(dendrites: Vec<Dendrite>) -> Self{
        Self{
            dendrites,
            potentialDendrites: Vec::new(),
            value: 0.0,
        }
    }

    pub fn activate(&mut self, neuronList: &Vec<Neuron>, priorFireList: &Vec<i64>, dopamine: bool, isLearning: bool) -> bool {
        // Gather activations
        for a in priorFireList{
            let activeNeuron: i64 = *a;
            if let Some(&dendrite) = self.dendrites.iter().find(|d| d.target == activeNeuron){
                self.value += dendrite.strength * OUTPUT_AMT;
            }
        }

        // Adjust and create dendrites
        if (isLearning){
            self.dendrites.retain_mut(|dendrite| {
                dendrite.adjust_strength(priorFireList, dopamine);
                dendrite.strength > 0.0
            });
            
            let mut i: i64 = self.potentialDendrites.len();
            while i > 0 {
                i -= 1;
                let dendrite = &mut self.potentialDendrites[i];
                dendrite.adjust_strength(priorFireList, dopamine);
                
                if dendrite.strength <= 0.0 {
                    self.potentialDendrites.remove(i);
                } else if dendrite.strength >= POTENTIAL_DENDRITE_ACCEPT_THRESHOLD {
                    self.dendrites.push(self.potentialDendrites.remove(i));
                }
            }

            for f in priorFireList{
                let fired: i64 = *f;
                if !self.dendrites.iter().any(|d| d.target == fired) && !self.potentialDendrites.iter().any(|d| d.target == fired){
                    self.potentialDendrites.push(Dendrite::new(fired));
                }
            }
        }    
        
        
        //Determine activation
        if self.value >= TRIGGER_THRESHOLD {
            self.value = 0.0;
            return true;
        }
        
        if self.value > 0.0 {
            self.value = self.value * BASELINE_RETURN_SPEED;
        }


        false
    }
}