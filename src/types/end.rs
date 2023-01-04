use serde::{Serialize, Deserialize};
use crate::types::player::PublicPlayer;

#[derive(Serialize, Deserialize, Debug)]
pub struct EndOfGame{
    EndOfGame: EndOfGameData
}

#[derive(Serialize, Deserialize, Debug)]
struct EndOfGameData{
    leader_board: Vec<PublicPlayer>
}