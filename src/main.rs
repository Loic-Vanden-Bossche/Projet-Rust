#![allow(non_snake_case)]

extern crate core;

mod types;
mod function;
mod challenges;

use log::{info, error, LevelFilter};
use simplelog::{ColorChoice, Config, TerminalMode};
use crate::function::args::parse_args;
use crate::function::connect::connect;
use crate::function::round::{challenge, end_of_round, get_player, start_round};
use crate::types::end::EndOfGame;
use crate::types::error::{RoundStartErrorEnum};

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
		let plb = match start_round(&stream) {
			Ok(val) => {
				info!("Round start");
				val
			}
			Err(err) => {
				match err.reason {
					RoundStartErrorEnum::EndOfGame(eog) => {
						info!("End of game");
						end = eog;
					}
					RoundStartErrorEnum::ReadError => {
						error!("Error on start round");
						return;
					}
				}
				break;
			}
		};
		let top1 = match get_player(&plb.PublicLeaderBoard) {
			Some(val) => {
				info!("Current best player is : {}", val.name);
				val
			}
			None => {
				error!("No player in leader board");
				return;
			}
		};
		let _sum = match challenge(&stream, &top1) {
			Some(val) => {
				info!("Round done");
				val
			}
			None => {
				match end_of_round(&stream) {
					Some(val) => {
						info!("End of round");
						val
					}
					None => {
						error!("Error on end of round");
						return;
					}
				}
			}
		};
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