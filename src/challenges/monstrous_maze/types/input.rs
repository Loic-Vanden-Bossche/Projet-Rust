use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u32,
}
