use std::ops::Add;
use log::{info, error};
use crate::function::connect::connect;
use crate::function::round::{get_player, round};
use crate::types::end::EndOfGame;
use crate::types::error::RoundErrorReason;

fn make_url(host: Option<String>, port: u32) -> String{
    match host {
        Some(host) => { host }
        None => { "localhost".to_string() }
    }.add(":").add(port.to_string().as_str())
}

pub fn game(host: Option<String>, port: u32, name: String){
    let stream = match connect(make_url(host, port), &name) {
        Some(s) => {
            info!("Connected");
            s
        }
        None => {
            error!("Error while connecting");
            return;
        }
    };
    let end: EndOfGame;
    loop {
        match round(&stream, &name) {
            Ok( sum ) => {
                info!("Challenge completed: {}", sum.RoundSummary.challenge)
            }
            Err(e) => {
                match e.reason {
                    RoundErrorReason::EndOfGame(eog) => {
                        end = eog;
                        break;
                    }
                    RoundErrorReason::StartError => {
                        error!("Error starting a round");
                        return;
                    }
                    RoundErrorReason::LeaderBoardError => {
                        error!("Error getting leader board");
                        return;
                    }
                    RoundErrorReason::EndError => {
                        error!("Error ending a round");
                        return;
                    }
                }
            }
        }
    }
    let top1 = match get_player(&end.EndOfGame.leader_board, &name, false) {
        Some(val) => { val }
        None => {
            error!("No player on leaderboard");
            return;
        }
    };
    info!("Player {} win with {} point! GG", top1.name, top1.score);
}