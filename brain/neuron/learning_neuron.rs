use crate::brain::neuron::base_neuron::BaseNeuron;
use crate::brain::neuron::dendrite::Dendrite;
use crate::constants::{
    BASELINE_REGRESSION_SPEED, 
	POTENTIAL_DENDRITE_ACCEPT_THRESHOLD, 
	TOTAL_NEURONS, 
	TRIGGER_THRESHOLD, 
	DOPAMINE_THRESHOLD
};

pub struct LearningNeuron{
    pub dendrites: Vec<Dendrite>,
    pub potential_dendrites: Vec<Dendrite>,
    pub value: f32,
}

impl LearningNeuron{
    pub fn new(dendrite_targets: &Vec<usize>) -> Self{
        let mut dendrites = (0..TOTAL_NEURONS).map(|dt| Dendrite::new(dt, true)).collect::<Vec<Dendrite>>();
		let potential_dendrites = dendrites.extract_if(.., |d| !dendrite_targets.contains(&d.target)).collect::<Vec<Dendrite>>();
        dendrites.iter_mut().for_each(|d| d.correlation = Some(1.0));
		Self{
            dendrites,
            potential_dendrites,
            value: 0.0,
        }
    }


	fn reorg_dendrites(&mut self) {
		// Downgrade weak dendrites
        let downgraded_dendrites: Vec<Dendrite> = self.dendrites.extract_if(.., |d| {
			if let Some(correlation) = d.correlation {
				correlation < POTENTIAL_DENDRITE_ACCEPT_THRESHOLD && correlation > -POTENTIAL_DENDRITE_ACCEPT_THRESHOLD
			} else {
				false
			}
        }).collect();

        // Upgrade strong dendrites
        let upgraded_dendrites: Vec<Dendrite> = self.potential_dendrites.extract_if(.., |d| {
			if let Some(correlation) = d.correlation {
				correlation > POTENTIAL_DENDRITE_ACCEPT_THRESHOLD || correlation < -POTENTIAL_DENDRITE_ACCEPT_THRESHOLD
			} else {
				false
			}
        }).collect();

        // Place moved dendrites
        self.dendrites.extend(upgraded_dendrites);
        self.potential_dendrites.extend(downgraded_dendrites);
	}


    pub fn activate(&mut self, prior_activations: &[bool;TOTAL_NEURONS], dopamine: f32) -> bool {
        // Gather inputs
        for dendrite in &self.dendrites{
            self.value += dendrite.get_output(prior_activations);
        }
		let active = self.value >= TRIGGER_THRESHOLD;

		// Adjust correlations
		for dendrite in &mut self.dendrites{
			dendrite.adjust_correlation(prior_activations, active);
		}
		for dendrite in &mut self.potential_dendrites{
			dendrite.adjust_correlation(prior_activations, active);
		}

		if dopamine > DOPAMINE_THRESHOLD || dopamine < -DOPAMINE_THRESHOLD {
			for dendrite in &mut self.dendrites{
				dendrite.dopamine_adjust(dopamine);
			}
			
			self.reorg_dendrites();
		}

		// Determine output and regress to mean
        if active { self.value = 0.0; }
        self.value *= 1.0 - BASELINE_REGRESSION_SPEED;

		active
    }
}


impl LearningNeuron{
    pub fn from_base(n: BaseNeuron) -> Self{
        let mut dendrites = n.dendrites;
        let potential_dendrites = dendrites.extract_if(.., |d| d.strength < 0.15).collect();

        Self { 
            dendrites, 
            potential_dendrites, 
            value: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a neuron with dendrites to targets 1 and 2, each with a strong
    /// correlation (above the accept threshold so they stay in `dendrites`).
    fn neuron_with_correlated_dendrites() -> LearningNeuron {
        let mut neuron = LearningNeuron::new(&vec![1, 2]);
        assert_eq!(neuron.dendrites.len(), 2);
        for d in &mut neuron.dendrites {
            d.correlation = Some(0.6);
        }
        neuron
    }

    #[test]
    fn dopamine_above_threshold_updates_dendrites() {
        let mut neuron = neuron_with_correlated_dendrites();
        let before: Vec<f32> = neuron.dendrites.iter().map(|d| d.strength).collect();

        neuron.activate(&[false; TOTAL_NEURONS], DOPAMINE_THRESHOLD + 1.0);

        assert_eq!(neuron.dendrites.len(), before.len());
        for (d, old) in neuron.dendrites.iter().zip(before) {
            assert!(d.strength > old, "positive dopamine should strengthen dendrite");
        }
    }

    #[test]
    fn negative_dopamine_weakens_dendrites() {
        let mut neuron = neuron_with_correlated_dendrites();
        let before: Vec<f32> = neuron.dendrites.iter().map(|d| d.strength).collect();

        neuron.activate(&[false; TOTAL_NEURONS], -(DOPAMINE_THRESHOLD + 1.0));

        assert_eq!(neuron.dendrites.len(), before.len());
        for (d, old) in neuron.dendrites.iter().zip(before) {
            assert!(d.strength < old, "negative dopamine should weaken dendrite");
        }
    }

    #[test]
    fn dopamine_below_threshold_leaves_dendrites_unchanged() {
        let mut neuron = neuron_with_correlated_dendrites();
        let before: Vec<f32> = neuron.dendrites.iter().map(|d| d.strength).collect();

        neuron.activate(&[false; TOTAL_NEURONS], DOPAMINE_THRESHOLD * 0.5);

        assert_eq!(neuron.dendrites.len(), before.len());
        for (d, old) in neuron.dendrites.iter().zip(before) {
            assert_eq!(d.strength, old);
        }
    }
}