mod design_patterns;
use design_patterns::behavioral::{
    chain_of_responsibility::chain_of_responsibility, strategy::strategy,
};

fn main() {
    println!("####### Strategy ####### ");
    strategy();
    println!("####### Chain of responsibility ####### ");
    chain_of_responsibility();
}
