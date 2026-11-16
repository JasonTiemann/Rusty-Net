mod neuron;
mod constants;
use crate::neuron::Dendrite;


fn main() {
    let d = Dendrite::new(1);

    println!("{}", d.strength);
    println!("{}", d.target);
}