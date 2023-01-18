use std::net::TcpStream;
use log::{error, info};
use serde_json::to_string;
use simplelog::debug;
use crate::challenges::hash_cash::hash_cash;
use crate::challenges::monstrous_maze::challenge::monstrous_maze;
use crate::function::stream::{read_from_stream, write_to_stream};
use crate::types::challenge::{Challenge, ChallengeAnswer, ChallengeEnum, ChallengeResult, ChallengeResultData};
use crate::types::error::{RoundError, RoundStartError, RoundStartErrorEnum};
use crate::types::error::RoundErrorReason::{EndError, EndOfGame, LeaderBoardError, StartError};
use crate::types::error::RoundStartErrorEnum::{EndOfGame as EndOfGameError, ReadError};
use crate::types::player::{PublicLeaderBoard, PublicPlayer};
use crate::types::round::RoundSummary;

pub fn start_round(stream: &TcpStream) -> Result<PublicLeaderBoard, RoundStartError>{
    match read_from_stream(&stream) {
        Ok(val) => {
            debug!("Round Start");
            Ok(val)
        }
        Err(e) => {
            debug!("Round didn't start as expected");
            if e.id == 1 {
                match serde_json::from_str(&*e.text) {
                    Ok(val) => {
                        debug!("End of Game");
                        Err(RoundStartError{reason: EndOfGameError(val)})
                    }
                    Err(e) => {
                        error!("Unexpected value, {e}");
                        Err(RoundStartError{reason: ReadError})
                    }
                }
            }else{
                error!("Error on round start");
                Err(RoundStartError{reason: ReadError})
            }
        }
    }
}

pub fn challenge(stream: &TcpStream, next: &PublicPlayer) -> Option<RoundSummary>{
    loop {
        let challenge: Challenge = match read_from_stream(&stream) {
            Ok(val) => {
                debug!("Successfully retrieve challenge");
                info!("I play");
                val
            }
            Err(e) => {
                debug!("No challenge");
                return if e.id == 1 {
                    match serde_json::from_str(&e.text) {
                        Ok(val) => {
                            debug!("Round summary received");
                            info!("Other player played");
                            Some(val)
                        }
                        Err(_) => {
                            error!("Invalid round data received");
                            None
                        }
                    }
                } else {
                    error!("Error reading current round data");
                    return None;
                }
            }
        };
        let answer = match challenge.Challenge {
            ChallengeEnum::MD5HashCash(input) => {
                info!("Playing MD5HashCash");
                ChallengeAnswer::MD5HashCash(hash_cash(input))
            }
            ChallengeEnum::MonstrousMaze(input) => {
                info!("Playing MonstrousMaze");
                ChallengeAnswer::MonstrousMaze(monstrous_maze(input))
            }
        };
        let result = ChallengeResult {
            ChallengeResult: ChallengeResultData {
                next_target: next.name.clone(),
                answer
            }
        };
        match to_string(&result) {
            Ok(text) => {
                if write_to_stream(stream, text) {
                    debug!("Challenge result send")
                } else {
                    error!("Error sending challenge result")
                }
            }
            Err(e) => {
                debug!("Error on parsing challenge result: {e}");
            }
        }
    }
}

pub fn get_player(plb: &Vec<PublicPlayer>) -> Option<&PublicPlayer> {
    let mut top1: &PublicPlayer = match &plb.get(0) {
        Some(val) => {
            val
        }
        None => {
            error!("Invalid list");
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
            debug!("Retrieve round summary");
            Some(val)
        }
        Err(_) => {
            error!("Error when retrieving end of round");
            None
        }
    }
}

pub fn round(stream: &TcpStream) -> Result<RoundSummary, RoundError>{
    let plb = match start_round(&stream) {
        Ok(val) => {
            info!("Round start");
            val
        }
        Err(err) => {
            return match err.reason {
                RoundStartErrorEnum::EndOfGame(eog) => {
                    info!("End of game");
                    Err(RoundError { reason: EndOfGame(eog) })
                }
                ReadError => {
                    error!("Error on start round");
                    Err(RoundError { reason: StartError })
                }
            }
        }
    };
    let top1 = match get_player(&plb.PublicLeaderBoard) {
        Some(val) => {
            info!("Current best player is : {}", val.name);
            val
        }
        None => {
            error!("No player in leader board");
            return Err(RoundError { reason: LeaderBoardError });
        }
    };
    match challenge(&stream, &top1) {
        Some(val) => {
            info!("Round done");
            Ok(val)
        }
        None => {
            error!("Error on end of round");
            Err(RoundError { reason: EndError })
        }
    }
}