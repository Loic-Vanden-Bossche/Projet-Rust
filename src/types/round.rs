use serde::{Serialize, Deserialize};
use crate::types::challenge::ReportedChallengeResult;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoundSummary{
    pub RoundSummary: RoundSummaryData
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoundSummaryData{
    pub challenge: String,
    pub chain: Vec<ReportedChallengeResult>
}