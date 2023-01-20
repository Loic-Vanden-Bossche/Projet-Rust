use crate::types::end::EndOfGame;
use crate::types::round::RoundSummary;

pub struct ReadError{
    pub id: i32,
    pub text: String
}

pub enum RoundStartErrorEnum{
    ReadError,
    EndOfGame(EndOfGame)
}

pub struct RoundStartError{
    pub reason: RoundStartErrorEnum
}

pub struct RoundError {
    pub reason: RoundErrorReason
}

pub enum RoundErrorReason {
    EndOfGame(EndOfGame),
    StartError,
    LeaderBoardError,
    EndError
}

pub enum ChallengeError {
    ChallengeInput,
    EndOfRound(RoundSummary)
}