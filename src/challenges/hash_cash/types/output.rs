use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashOutput {
    // Seed used to solve the challenge
    pub(crate) seed: u64,
    // hashcode found using seed + message
    pub(crate) hashcode: String,
}