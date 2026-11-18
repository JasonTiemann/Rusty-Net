pub struct LearningNeuron{
    dendrites: Vec<Dendrite>,
    potential_dendrites: Vec<Dendrite>,
    output_history: [usize; NEURON_HISTORY_LENGTH],
    pub value: f32,
}

impl LearningNeuron{
    pub fn new() -> Self{
        Self{
            dendrites: vec![],
            potential_dendrites: vec![],
            output_history: [0; NEURON_HISTORY_LENGTH],
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

        // Learning logic
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
