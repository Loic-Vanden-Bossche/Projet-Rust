use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Map {
    pub maze: Vec<Vec<char>>,
    pub player: (usize, usize),
    pub exit: (usize, usize),
}
