#![allow(non_snake_case)]

mod types;
mod function;

use crate::function::args::parse_args;
use crate::function::connect::connect;
use crate::function::round::{challenge, end_of_round, get_player, start_round};
use crate::function::stream::{read_from_stream, write_to_stream};
use crate::types::challenge::{Challenge, ChallengeAnswer, ChallengeResult, ChallengeResultData, MD5HashCashOutput};
use crate::types::end::EndOfGame;
use crate::types::error::{RoundStartError, RoundStartErrorEnum};
use crate::types::player::{PublicLeaderBoard, PublicPlayer};
use crate::types::round::RoundSummary;

fn main() {
	let (name, ip) = parse_args();
	let stream = match connect(ip, name) {
		Ok(s) => {
			println!("Connecté");
			s
		}
		Err(err) => {
			println!("Erreur lors de la connection {}", err.coucou);
			return;
		}
	};
	let end: EndOfGame;
	loop {
		let plb = match start_round(&stream) {
			Ok(val) => {
				val
			}
			Err(err) => {
				match err.reason {
					RoundStartErrorEnum::EndOfGame(eog) => {
						end = eog;
					}
					RoundStartErrorEnum::ReadError => {
						println!("Error on start round");
						return;
					}
				}
				break;
			}
		};
		let top1 = get_player(&plb.PublicLeaderBoard);
		let sum = match challenge(&stream, &top1) {
			Some(val) => {
				val
			}
			None => {
				match end_of_round(&stream) {
					Some(val) => {
						val
					}
					None => {
						println!("Error");
						return;
					}
				}
			}
		};
	}
	println!("{}", serde_json::to_string(&end).unwrap());
	let top1 = get_player(&end.EndOfGame.leader_board);
	println!("Player {} win with {} point! GG", top1.name, top1.score);
}