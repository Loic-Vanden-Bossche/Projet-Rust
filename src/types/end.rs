use serde::{Serialize, Deserialize};
use crate::types::player::PublicPlayer;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndOfGame{
    pub EndOfGame: EndOfGameData
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndOfGameData{
    pub leader_board: Vec<PublicPlayer>
}