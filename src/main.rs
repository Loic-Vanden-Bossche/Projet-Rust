#![allow(non_snake_case)]

mod types;
mod function;

use crate::function::args::parse_args;
use crate::function::connect::connect;
use crate::function::round::{challenge, start_round};
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
					}
				}
				return;
			}
		};
		let mut top1: &PublicPlayer = plb.PublicLeaderBoard.get(0).unwrap().clone();
		for p in plb.PublicLeaderBoard {
			if top1.score > p.score {
				top1 = &p;
			}
		}
		challenge(&stream, &top1);
		let sum: RoundSummary = match read_from_stream(&stream) {
			Ok(val) => {
				val
			}
			Err(_) => {
				println!("Error");
				return;
			}
		};
		println!("{}", serde_json::to_string(&sum).unwrap());
	}
	println!("{}", serde_json::to_string(&end).unwrap());
}