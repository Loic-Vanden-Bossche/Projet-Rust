use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub version: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome {
    pub Welcome: Version
}