#![allow(non_snake_case)]

use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use byteorder::{BigEndian, ReadBytesExt};

use serde::{Serialize, Deserialize};
use crate::SubscribeError::InvalidName;

#[derive(Serialize, Deserialize, Debug)]
struct Version {
	version: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Welcome {
	Welcome: Version
}

#[derive(Serialize, Deserialize, Debug)]
struct Name{
	name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Subscribe {
	Subscribe: Name
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum SubscribeError{
	AlreadyRegistered, InvalidName
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum SubscribeResultEnum{
	Ok,
	Err(SubscribeError)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SubscribeResult{
	SubscribeResult: SubscribeResultEnum
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
				InvalidName => {
					println!("Invalid Name");
					Err(Error { coucou: 5 })
				}
				_AlreadyRegistered => {
					Err(Error { coucou: 6 })
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
	match connect(ip, name) {
		Ok(_) => {
			println!("Connecté");
		}
		Err(err) => {
			println!("Erreur lors de la connection {}", err.coucou);
		}
	}
}
