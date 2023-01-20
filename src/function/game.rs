use std::ops::Add;
use log::{info, error, debug, LevelFilter};
use simplelog::{ColorChoice, Config, TerminalMode};
use crate::function::connect::connect;
use crate::function::round::{get_player, round};
use crate::types::end::EndOfGame;
use crate::types::error::RoundErrorReason;

pub fn make_url(host: Option<String>, port: u32) -> String{
    match host {
        Some(host) => { host }
        None => { "localhost".to_string() }
    }.add(":").add(port.to_string().as_str())
}

pub fn game(host: Option<String>, port: u32, name: Option<String>, debug: LevelFilter){
    match simplelog::TermLogger::init(debug, Config::default(), TerminalMode::Mixed, ColorChoice::Always) {
        Ok(_) => { debug!("Logger loaded") }
        Err(err) => {
            println!("Error on loading logger: {err}")
        }
    }
    info!("No UI");
    let name = if let Some(val) = name{
        val
    }else{
        error!("Name required without UI");
        return;
    };
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