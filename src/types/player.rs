use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicPlayer{
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicLeaderBoard{
    pub PublicLeaderBoard: Vec<PublicPlayer>
}