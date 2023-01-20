use log::debug;
use super::types::{input::MD5HashCashInput, output::MD5HashCashOutput};

pub fn hash_cash(input: MD5HashCashInput) -> MD5HashCashOutput {
    debug!("complexity: {}\nmessage: {}", input.complexity, input.message);
    MD5HashCashOutput{ seed: 666, hashcode: "NIKKKK".to_string()}
}