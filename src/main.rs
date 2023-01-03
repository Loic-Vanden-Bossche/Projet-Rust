use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use byteorder::{BigEndian, ReadBytesExt};

use serde::{Serialize, Deserialize};

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
	let stream = TcpStream::connect("127.0.0.1:7878");
	let hello = b"\"Hello\"";
	let mut size_w = 7_u32.to_be_bytes();
	match stream {
		Ok(mut stream) => {
			println!("Connected");
			stream.write(&(size_w)).expect("PANIKKK");
			stream.write(hello).expect("PANIKKK");
			read_from_stream(stream)
		}
		Err(_) => {
			println!("Failed to connect");
		}
	}
}
