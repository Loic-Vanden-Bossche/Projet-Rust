#![allow(non_snake_case)]

mod types;

use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use byteorder::{BigEndian, ReadBytesExt};

use serde::{Serialize, Deserialize};
use crate::types::subscribe::{Name, Subscribe, SubscribeError, SubscribeResult, SubscribeResultEnum};

#[derive(Serialize, Deserialize, Debug)]
struct Version {
	version: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Welcome {
	Welcome: Version
}

#[derive(Serialize, Deserialize, Debug)]
struct PublicPlayer{
	name: String,
	stream_id: String,
	score: i32,
	steps: u32,
	is_active: bool,
	total_used_time: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct PublicLeaderBoard{
	PublicLeaderBoard: Vec<PublicPlayer>
}

#[derive(Serialize, Deserialize, Debug)]
enum ChallengeEnum{
	MD5HashCash(MD5HashCashInput)
}

#[derive(Serialize, Deserialize, Debug)]
struct MD5HashCashInput{
	complexity: u32,
	message: String
}

#[derive(Serialize, Deserialize, Debug)]
struct MD5HashCashOutput{
	seed: u64,
	hashcode: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Challenge{
	Challenge: ChallengeEnum
}

#[derive(Serialize, Deserialize, Debug)]
enum ChallengeAnswer{
	MD5HashCash(MD5HashCashOutput)
}

#[derive(Serialize, Deserialize, Debug)]
struct ChallengeResult{
	ChallengeResult: ChallengeResultData
}

#[derive(Serialize, Deserialize, Debug)]
struct ChallengeResultData{
	answer: ChallengeAnswer,
	next_target: String
}

struct Error{
	coucou: i32
}

fn read_from_stream<T:for<'a> Deserialize<'a>>(mut stream: &TcpStream) -> Result<T, Error>{
	match stream.read_u32::<BigEndian>() {
		Ok(size_r) => {
			let mut buf = vec![0u8; size_r as usize];
			match stream.read_exact(& mut buf) {
				Ok(_) => {
					let text = from_utf8(&buf).unwrap();
					Ok(serde_json::from_str(text).unwrap())
				}
				Err(e) => {
					println!("Erreur à la lecture: {}", e);
					Err(Error{ coucou: 1})
				}
			}
		}
		Err(e) => {
			println!("Failed to read data: {}", e);
			Err(Error{ coucou: 2})
		}
	}
}

fn write_to_stream(mut stream: &TcpStream, message: String){
	let size: u32;
	match u32::try_from(message.len()) {
		Ok(val) => {
			size = val;
		}
		Err(_) => {
			println!("Erreur message trop long");
			return;
		}
	}
	let size_b = size.to_be_bytes();
	stream.write(&size_b).expect("AHHHH");
	let mess = message.into_bytes();
	stream.write(&mess).expect("OULA");
}

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
	let welcome: Result<Welcome, Error> = read_from_stream(&stream);
	let sub_res = match welcome {
		Ok(val) => {
			if val.Welcome.version != 1 {
				println!("Unsupported version");
				return Err(Error{coucou: 4});
			}
			let sub = Subscribe{Subscribe: Name{ name }};
			write_to_stream(&stream, serde_json::to_string(&sub).unwrap());
			let res: Result<SubscribeResult, Error> = read_from_stream(&stream);
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
	let plb: PublicLeaderBoard = match read_from_stream(&stream) {
		Ok(val) => {
			val
		}
		Err(_) => {
			println!("nope");
			return;
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
	let test = ChallengeResult{ChallengeResult: ChallengeResultData{next_target: t.name.clone(), answer: ChallengeAnswer::MD5HashCash(MD5HashCashOutput{hashcode: "Coucou".to_string(), seed: 0})}};
	write_to_stream(&stream, serde_json::to_string(&test).unwrap());
}
