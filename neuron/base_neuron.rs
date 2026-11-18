pub struct BaseNeuron{
    dendrites: Vec<Dendrite>,
    pub value: f32,
}


impl BaseNeuron{
    pub fn new() -> Self{
        Self{
            dendrites: vec![],
            value: 0.0,
        }
    }


    pub fn activate(&mut self, neuronList: &Vec<Neuron>, priorFireList: &Vec<i64>) -> bool {
        // Gather activations
        for a in priorFireList{
            let activeNeuron: i64 = *a;
            if let Some(&dendrite) = self.dendrites.iter().find(|d| d.target == activeNeuron){
                self.value += dendrite.strength * OUTPUT_AMT;
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

