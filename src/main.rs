#![allow(non_snake_case)]

use std::arch::x86_64::_mm_rcp_ps;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use byteorder::{BigEndian, ReadBytesExt};

use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize, Debug)]
enum SubscribeError{
	AlreadyRegistered, InvalidName
}

#[derive(Serialize, Deserialize, Debug)]
enum SubscribeResultEnum{
	Ok,
	Err(SubscribeError)
}

#[derive(Serialize, Deserialize, Debug)]
struct SubscribeResult{
	SubscribeResult: SubscribeResultEnum
}

struct Error{
	coucou: i32
}

fn read_from_stream<T:for<'a> Deserialize<'a>>(mut stream: &TcpStream) -> Result<T, Error>{
	match stream.read_u32::<BigEndian>() {
		Ok(size_r) => {
			println!("Size: {}", &size_r);
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
	let mut size: u32;
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

fn main() {
	let stream = TcpStream::connect("127.0.0.1:7878");
	let hello = b"\"Hello\"";
	let mut size_w = 7_u32.to_be_bytes();
	match stream {
		Ok(mut stream) => {
			println!("Connected");
			stream.write(&(size_w)).expect("PANIKKK");
			stream.write(hello).expect("PANIKKK");
			let welcome: Result<Welcome, Error> = read_from_stream(&stream);
			match welcome {
				Ok(val) => {
					if val.Welcome.version == 1 {
						let sub = Subscribe{Subscribe: Name{ name: "macaron".to_string() }};
						write_to_stream(&stream, serde_json::to_string(&sub).unwrap());
						let res: Result<SubscribeResult, Error> = read_from_stream(&stream);
						match res {
							Ok(val) => {
								println!("{}", serde_json::to_string(&val).unwrap())
							}
							Err(_) => {
								println!("D'où")
							}
						}
					}else{
						println!("Unsupported version");
					}
				}
				Err(_) => {
					println!("MERDE")
				}
			}
		}
		Err(_) => {
			println!("Failed to connect");
		}
	}
}
