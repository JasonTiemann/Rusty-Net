mod neuron;
mod constants;
mod brain;
use crate::brain::Brain;


fn main() {
    let n = Brain::new(5, 1, 1, true);
}