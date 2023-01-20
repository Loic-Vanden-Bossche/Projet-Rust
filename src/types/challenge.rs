use serde::{Serialize, Deserialize};
use crate::challenges::hash_cash::types::input::MD5HashCashInput;
use crate::challenges::hash_cash::types::output::MD5HashCashOutput;
use crate::challenges::monstrous_maze::types::input::MonstrousMazeInput;
use crate::challenges::monstrous_maze::types::output::MonstrousMazeOutput;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Challenge{
    pub Challenge: ChallengeEnum
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChallengeEnum{
    MD5HashCash(MD5HashCashInput),
    MonstrousMaze(MonstrousMazeInput)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer{
    MD5HashCash(MD5HashCashOutput),
    MonstrousMaze(MonstrousMazeOutput)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeResult{
    pub ChallengeResult: ChallengeResultData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeResultData{
    pub answer: ChallengeAnswer,
    pub next_target: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChallengeValue{
    Unreachable,
    Timeout,
    BadResult {used_time: f64, next_target: String},
    Ok {used_time: f64, next_target: String}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportedChallengeResult{
    name: String,
    value: ChallengeValue
}