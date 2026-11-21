// pub const REFRACTORY_PERIOD: i8 = 5;
pub const INPUT_NEURONS: usize = 2;
pub const OUTPUT_NEURONS: usize = 1;
pub const TOTAL_NEURONS: usize = 10;

pub const OUTPUT_SAMPLE: usize = 10;

pub const TRIGGER_THRESHOLD: f32 = 5.0;
pub const INPUT_NEURON_BASE_THRESHOLD: f32 = 0.5;
pub const BASELINE_REGRESSION_SPEED: f32 = 0.5;

pub const DENDRITE_STRENGTHEN_FACTOR: f32 = 0.0001;
pub const DENDRITE_GEN_STRENGTH: f32 = 1.0;
pub const DENDRITE_MAX_STRENGTH: f32 = 5.0;
pub const POTENTIAL_DENDRITE_ACCEPT_THRESHOLD: f32 =  0.25;

pub const DOPAMINE_MULTI: f32 = 10.0;
pub const DOPAMINE_TICK: usize = 10;
pub const DOPAMINE_REGRESS_RATE: f32 = 0.01;

pub const NEURON_HISTORY_LENGTH: usize = 50;