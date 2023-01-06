use crate::types::end::EndOfGame;

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

pub struct Error{
    pub coucou: i32
}