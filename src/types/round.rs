use serde::{Serialize, Deserialize};
use crate::types::challenge::ReportedChallengeResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundSummary{
    RoundSummary: RoundSummaryData
}

#[derive(Serialize, Deserialize, Debug)]
struct RoundSummaryData{
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}