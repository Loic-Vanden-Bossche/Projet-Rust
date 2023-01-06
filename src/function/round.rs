use std::net::TcpStream;
use serde_json::to_string;
use crate::function::stream::{read_from_stream, write_to_stream};
use crate::types::challenge::{Challenge, ChallengeAnswer, ChallengeEnum, ChallengeResult, ChallengeResultData, MD5HashCashOutput};
use crate::types::end::EndOfGame;
use crate::types::error::{RoundStartError, RoundStartErrorEnum};
use crate::types::error::RoundStartErrorEnum::{EndOfGame as EndOfGameError, ReadError};
use crate::types::player::{PublicLeaderBoard, PublicPlayer};
use crate::types::round::RoundSummary;

pub fn start_round(stream: &TcpStream) -> Result<PublicLeaderBoard, RoundStartError>{
    match read_from_stream(&stream) {
        Ok(val) => {
            Ok(val)
        }
        Err(e) => {
            if e.id == 1 {
                Err(RoundStartError{reason: EndOfGameError(serde_json::from_str(&*e.text).unwrap())})
            }else{
                println!("Error start");
                Err(RoundStartError{reason: ReadError})
            }
        }
    }
}

pub fn challenge(stream: &TcpStream, next: &PublicPlayer) -> Option<RoundSummary>{
    let challenge: Challenge = match read_from_stream(&stream) {
        Ok(val) => {
            val
        }
        Err(e) => {
            if e.id == 1 {
                return Some(serde_json::from_str(&e.text).unwrap())
            }
            println!("Dommage");
            return None;
        }
    };
    match challenge.Challenge {
        ChallengeEnum::MD5HashCash(input) => {
            let test = ChallengeResult { ChallengeResult: ChallengeResultData { next_target: next.name.clone(), answer: ChallengeAnswer::MD5HashCash(MD5HashCashOutput { hashcode: "Coucou".to_string(), seed: 0 }) } };
            write_to_stream(&stream, to_string(&test).unwrap());
        }
    }
    return None
}

pub fn get_player(plb: &Vec<PublicPlayer>) -> &PublicPlayer{
    let mut top1: &PublicPlayer = &plb.get(0).unwrap();
    for p in plb {
        if top1.score < p.score {
            top1 = p;
        }
    }
    top1
}

pub fn end_of_round(stream: &TcpStream) -> Option<RoundSummary>{
    match read_from_stream(stream) {
        Ok(val) => {
            Some(val)
        }
        Err(_) => {
            println!("Error");
            None
        }
    }
}