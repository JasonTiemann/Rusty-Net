mod constants;
mod brain;
use crate::brain::Brain;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};


fn main() {
    let mut brain = Brain::new(true);
    brain.start(|x| {print!("\r{:?}", x); io::stdout().flush().unwrap();});

    thread::sleep(Duration::from_secs(5));
    brain.update_inputs([0.25, 1.0]);
    
    thread::sleep(Duration::from_secs(5));
    brain.update_inputs([0.75, 0.25]);
    
    thread::sleep(Duration::from_secs(5));

    brain.stop();
}