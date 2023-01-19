use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MD5HashCashInput {
    // complexity in bits
    pub complexity: u32,
    // message to sign
    pub message: String,
}
