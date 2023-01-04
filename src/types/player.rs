use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicPlayer{
    pub name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicLeaderBoard{
    pub PublicLeaderBoard: Vec<PublicPlayer>
}