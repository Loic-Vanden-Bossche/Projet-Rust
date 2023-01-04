use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeEnum{
    MD5HashCash(MD5HashCashInput)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashInput{
    pub complexity: u32,
    pub message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashOutput{
    pub seed: u64,
    pub hashcode: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Challenge{
    pub Challenge: ChallengeEnum
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer{
    MD5HashCash(MD5HashCashOutput)
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

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeValue{
    Unreachable,
    Timeout,
    BadResult {used_time: f64, next_target: String},
    Ok {used_time: f64, next_target: String}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedChallengeResult{
    name: String,
    value: ChallengeValue
}