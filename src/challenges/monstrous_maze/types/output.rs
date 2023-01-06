use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonstrousMazeOutput {
    pub path: String,
}