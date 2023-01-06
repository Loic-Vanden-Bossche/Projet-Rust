use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use byteorder::{BigEndian, ReadBytesExt};
use serde::Deserialize;
use serde_json::to_string;
use crate::types::error::ReadError;

pub fn read_from_stream<T:for<'a> Deserialize<'a>>(mut stream: &TcpStream) -> Result<T, ReadError>{
    match stream.read_u32::<BigEndian>() {
        Ok(size_r) => {
            let mut buf = vec![0u8; size_r as usize];
            match stream.read_exact(& mut buf) {
                Ok(_) => {
                    let text = match from_utf8(&buf) {
                        Ok(val) => {
                            val
                        }
                        Err(_) => {
                            return Err(ReadError{id: 4, text: "".to_string()})
                        }
                    };
                    match serde_json::from_str(text) {
                        Ok(val) => {
                            Ok(val)
                        }
                        Err(_) => {
                            Err(ReadError{id: 1, text: text.to_string()})
                        }
                    }
                }
                Err(e) => {
                    println!("Erreur à la lecture: {}", e);
                    Err(ReadError{id: 2, text: "".to_string()})
                }
            }
        }
        Err(e) => {
            println!("Failed to read data: {}", e);
            Err(ReadError{id: 3, text: "".to_string()})
        }
    }
}

pub fn write_to_stream(mut stream: &TcpStream, message: String){
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
    match stream.write(&size_b) {
        Ok(_) => {
            let mess = message.into_bytes();
            match stream.write(&mess) {
                Ok(size) => {
                    println!("{size} bytes written")
                }
                Err(_) => {
                    println!("Error while writing message")
                }
            }
        }
        Err(_) => {
            println!("Error in writing message size");
        }
    }
}