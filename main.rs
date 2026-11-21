mod constants;
mod brain;
use crate::brain::Brain;
use crate::constants::{INPUT_NEURONS, OUTPUT_NEURONS};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

const IN_OUT: [([f32; 2], [f32; 1]); 4] = [
    ([0.0, 0.0], [0.0]),
    ([1.0, 0.0], [1.0]),
    ([0.0, 1.0], [1.0]),
    ([1.0, 1.0], [0.0]),
];


fn on_output(input: [f32; INPUT_NEURONS], output: [f32; OUTPUT_NEURONS]){
    print!("\r{:?}", output); io::stdout().flush().unwrap();
}

fn main() {
    let mut brain = Brain::new(true);
    brain.start(on_output);

    thread::sleep(Duration::from_secs(5));
    brain.update_inputs([0.25, 1.0]);
    
    thread::sleep(Duration::from_secs(5));
    brain.update_inputs([0.75, 0.25]);
    
    thread::sleep(Duration::from_secs(5));

    brain.stop();
}