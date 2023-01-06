use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    pub grid: String,
    pub endurance: u32,
}
