use super::types::{input::Input, output::Output};

pub fn hash_cash(input: Input) {
    println!("complexity: {}\nmessage: {}", input.complexity, input.message)
}