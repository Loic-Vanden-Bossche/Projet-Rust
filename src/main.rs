#![allow(non_snake_case)]

mod types;
mod function;

use std::env;
use std::net::TcpStream;

use crate::function::stream::{read_from_stream, write_to_stream};
use crate::types::challenge::{Challenge, ChallengeAnswer, ChallengeResult, ChallengeResultData, MD5HashCashOutput};
use crate::types::end::EndOfGame;
use crate::types::error::{Error, ReadError};
use crate::types::player::{PublicLeaderBoard};
use crate::types::round::RoundSummary;
use crate::types::subscribe::{Name, Subscribe, SubscribeError, SubscribeResult, SubscribeResultEnum};
use crate::types::welcome::Welcome;

fn connect(ip: String, name: String) -> Result<TcpStream, Error>{
	let res_stream = TcpStream::connect(ip);
	let stream = match res_stream {
		Ok(res) => {
			res
		}
		Err(_) => {
			println!("Failed to connect");
			return Err(Error{coucou: 4});
		}
	};
	write_to_stream(&stream, "\"Hello\"".to_string());
	let welcome: Result<Welcome, ReadError> = read_from_stream(&stream);
	let sub_res = match welcome {
		Ok(val) => {
			if val.Welcome.version != 1 {
				println!("Unsupported version");
				return Err(Error{coucou: 4});
			}
			let sub = Subscribe{Subscribe: Name{ name }};
			write_to_stream(&stream, serde_json::to_string(&sub).unwrap());
			let res: Result<SubscribeResult, ReadError> = read_from_stream(&stream);
			match res {
				Ok(val) => {
					val
				}
				Err(_) => {
					return Err(Error{coucou: 5})
				}
			}
		}
		Err(_) => {
			println!("MERDE");
			return Err(Error{coucou: 4});
		}
	};
	match sub_res.SubscribeResult {
		SubscribeResultEnum::Ok => {
			Ok(stream)
		}
		SubscribeResultEnum::Err(error) => {
			return match error {
				SubscribeError::InvalidName => {
					println!("Invalid Name");
					Err(Error { coucou: 5 })
				}
				SubscribeError::AlreadyRegistered => {
					println!("Already registered");
					Err(Error{coucou: 6})
				}
			}
		}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut name = "macaron".to_string();
	let mut ip = "127.0.0.1:7878".to_string();
	let mut next = 0;
	for arg in args {
		if arg == "--name" {
			next = 1;
		}
		if arg == "--ip" {
			next = 2;
		}
		if next == 1 {
			name = arg;
		}else if next == 2 {
			ip = arg
		}
	}
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
		let plb: PublicLeaderBoard = match read_from_stream(&stream) {
			Ok(val) => {
				val
			}
			Err(e) => {
				if e.id == 1 {
					end = serde_json::from_str(&*e.text).unwrap();
					break;
				}else{
					println!("Error start");
					return;
				}
			}
		};
		let test: Challenge = match read_from_stream(&stream) {
			Ok(val) => {
				val
			}
			Err(_) => {
				println!("Dommage");
				return;
			}
		};
		let t = plb.PublicLeaderBoard.get(0).unwrap();
		println!("{}", serde_json::to_string(&plb).unwrap());
		println!("{}", serde_json::to_string(&test).unwrap());
		let test = ChallengeResult { ChallengeResult: ChallengeResultData { next_target: t.name.clone(), answer: ChallengeAnswer::MD5HashCash(MD5HashCashOutput { hashcode: "Coucou".to_string(), seed: 0 }) } };
		write_to_stream(&stream, serde_json::to_string(&test).unwrap());
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