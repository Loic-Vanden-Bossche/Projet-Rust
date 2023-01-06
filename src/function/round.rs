use std::net::TcpStream;
use serde_json::to_string;
use crate::challenges::hash_cash::hash_cash;
use crate::challenges::monstrous_maze::challenge::monstrous_maze;
use crate::function::stream::{read_from_stream, write_to_stream};
use crate::types::challenge::{Challenge, ChallengeAnswer, ChallengeEnum, ChallengeResult, ChallengeResultData};
use crate::types::error::{RoundStartError};
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
                match serde_json::from_str(&*e.text) {
                    Ok(val) => {
                        Err(RoundStartError{reason: EndOfGameError(val)})
                    }
                    Err(_) => {
                        Err(RoundStartError{reason: ReadError})
                    }
                }
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
                return match serde_json::from_str(&e.text) {
                    Ok(val) => {
                        Some(val)
                    }
                    Err(_) => {
                        None
                    }
                }
            }
            println!("Dommage");
            return None;
        }
    };
    match challenge.Challenge {
        ChallengeEnum::MD5HashCash(input) => {
            let test = ChallengeResult { ChallengeResult: ChallengeResultData { next_target: next.name.clone(), answer: ChallengeAnswer::MD5HashCash(hash_cash(input)) } };
            write_to_stream(&stream, to_string(&test).unwrap());
        }
        ChallengeEnum::MonstrousMaze(input) => {
            let test = ChallengeResult { ChallengeResult: ChallengeResultData { next_target: next.name.clone(), answer: ChallengeAnswer::MonstrousMaze(monstrous_maze(input)) } };
            write_to_stream(&stream, to_string(&test).unwrap());
        }
    }
    return None
}

pub fn get_player(plb: &Vec<PublicPlayer>) -> Option<&PublicPlayer> {
    let mut top1: &PublicPlayer = match &plb.get(0) {
        Some(val) => {
            val
        }
        None => {
            return None
        }
    };
    for p in plb {
        if top1.score < p.score {
            top1 = p;
        }
    }
    Some(top1)
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