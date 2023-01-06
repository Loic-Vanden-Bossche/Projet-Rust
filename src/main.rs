#![allow(non_snake_case)]

extern crate core;

mod types;
mod function;
mod challenges;

use log::{info, error, LevelFilter};
use simplelog::{ColorChoice, Config, TerminalMode};
use crate::function::args::parse_args;
use crate::function::connect::connect;
use crate::function::round::{challenge, end_of_round, get_player, round, start_round};
use crate::types::end::EndOfGame;
use crate::types::error::{RoundErrorReason, RoundStartErrorEnum};

fn main() {
	simplelog::TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Always).unwrap();
	let (name, ip) = parse_args();
	let stream = match connect(ip, name) {
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
		match round(&stream) {
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
	let top1 = match get_player(&end.EndOfGame.leader_board) {
		Some(val) => { val }
		None => {
			error!("No player on leaderboard");
			return;
		}
	};
	info!("Player {} win with {} point! GG", top1.name, top1.score);
}