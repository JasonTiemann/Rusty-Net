mod constants;
mod brain;
use crate::brain::Brain;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

const in_out: [([f32; 2], [f32; 1]); 4] = [
    ([0.0, 0.0], [0.0]),
    ([1.0, 0.0], [1.0]),
    ([0.0, 1.0], [1.0]),
    ([1.0, 1.0], [0.0]),
];


fn onOutput(input: [f32; INPUT_NEURONS], output: [f32; OUTPUT_NEURONS]){
    print!("\r{:?}", output); io::stdout().flush().unwrap();

    let out_index = in_out.iter().position(|r| r[0] == input)
    let expected_output = in_out[out_index][1];

    let mut diff = 0.0;
}

fn main() {
    let mut brain = Brain::new(true);
    brain.start(onOutput);

    thread::sleep(Duration::from_secs(5));
    brain.update_inputs([0.25, 1.0]);
    
    thread::sleep(Duration::from_secs(5));
    brain.update_inputs([0.75, 0.25]);
    
    thread::sleep(Duration::from_secs(5));

    brain.stop();
}