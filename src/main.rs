mod challenges;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use byteorder::{BigEndian, ReadBytesExt};

use serde::{Serialize, Deserialize};

use crate::challenges::hash_cash::challenge;
use crate::challenges::hash_cash::types::{input::Input};

#[derive(Serialize, Deserialize, Debug)]
struct version{
	version: i32
}

impl version{
	/*fn to_string(&self) ->String {
		format!("{{\"version\":{}}}", self.version)
	}*/
}

#[derive(Serialize, Deserialize, Debug)]
struct Welcome {
	Welcome: version
}

impl Welcome {
	/*fn to_string(&self) -> String {
		format!("{{\"Welcome\":{}}}", self.Welcome.to_string())
	}*/
}

fn read_from_stream(mut stream: TcpStream){
	match stream.read_u32::<BigEndian>() {
		Ok(size_r) => {
			println!("Size: {}", &size_r);
			let mut buf = vec![0u8; size_r as usize];
			match stream.read_exact(& mut buf) {
				Ok(_) => {
					let text = from_utf8(&buf).unwrap();
					println!("{}", text);
					let test2: Welcome = serde_json::from_str(text).unwrap();
					println!("{}", serde_json::to_string(&test2).unwrap());

				}
				Err(e) => {
					println!("Erreur à la lecture: {}", e);
				}
			}
		}
		Err(e) => {
			println!("Failed to read data: {}", e);
		}
	}
}

fn main() {
	// let (name, ip) = parse_args();
	// let stream = match connect(ip, name) {
	// 	Ok(s) => {
	// 		println!("Connecté");
	// 		s
	// 	}
	// 	Err(err) => {
	// 		println!("Erreur lors de la connection {}", err.coucou);
	// 		return;
	// 	}
	// };
	// let end: EndOfGame;
	// loop {
	// 	let plb = match start_round(&stream) {
	// 		Ok(val) => {
	// 			val
	// 		}
	// 		Err(err) => {
	// 			match err.reason {
	// 				RoundStartErrorEnum::EndOfGame(eog) => {
	// 					end = eog;
	// 				}
	// 				RoundStartErrorEnum::ReadError => {
	// 					println!("Error on start round");
	// 				}
	// 			}
	// 			return;
	// 		}
	// 	};
	// 	let mut top1: &PublicPlayer = plb.PublicLeaderBoard.get(0).unwrap().clone();
	// 	for p in plb.PublicLeaderBoard {
	// 		if top1.score > p.score {
	// 			top1 = &p;
	// 		}
	// 	}
	// 	challenge(&stream, &top1);
	// 	let sum: RoundSummary = match read_from_stream(&stream) {
	// 		Ok(val) => {
	// 			val
	// 		}
	// 		Err(_) => {
	// 			println!("Error");
	// 			return;
	// 		}
	// 	};
	// 	println!("{}", serde_json::to_string(&sum).unwrap());
	// }
	// println!("{}", serde_json::to_string(&end).unwrap());

	challenge(Input{complexity: 3, message: "blabla".to_string()})
}