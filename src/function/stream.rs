use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use byteorder::{BigEndian, ReadBytesExt};
use log::{debug, error, trace};
use serde::Deserialize;
use crate::types::error::ReadError;

pub fn read_from_stream<T:for<'a> Deserialize<'a>>(mut stream: &TcpStream) -> Result<T, ReadError>{
    let in_size = match stream.read_u32::<BigEndian>() {
        Ok(size_r) => {
            debug!("{} bytes to read", size_r);
            size_r
        }
        Err(e) => {
            error!("Failed to read data: {}", e);
            return Err(ReadError { id: 3, text: "".to_string() })
        }
    };
    let mut buf = vec![0u8; in_size as usize];
    let text = match stream.read_exact(&mut buf) {
        Ok(_) => {
            match from_utf8(&buf) {
                Ok(val) => {
                    debug!("Read : \"{}\"", val);
                    val
                }
                Err(e) => {
                    error!("Error while reading message of length {in_size}: {e}");
                    return Err(ReadError { id: 4, text: "".to_string() })
                }
            }
        }
        Err(e) => {
            error!("Error while reading message: {e}");
            return Err(ReadError{id: 2, text: "".to_string()})
        }
    };
    match serde_json::from_str(text) {
        Ok(val) => {
            trace!("Successfully parsed");
            Ok(val)
        }
        Err(e) => {
            debug!("Bad structure to parse: {e}");
            Err(ReadError { id: 1, text: text.to_string() })
        }
    }
}

pub fn write_to_stream(mut stream: &TcpStream, message: String) -> bool{
    let size = match u32::try_from(message.len()) {
        Ok(val) => {
            debug!("{val} bytes to write");
            val
        }
        Err(_) => {
            error!("Message too long");
            return false;
        }
    }.to_be_bytes();
    match stream.write(&size) {
        Ok(_) => {
            let mess = message.into_bytes();
            match stream.write(&mess) {
                Ok(_) => {
                    debug!("Message send");
                    true
                }
                Err(e) => {
                    error!("Error while writing message: {e}");
                    false
                }
            }
        }
        Err(e) => {
            error!("Error in writing message size: {e}");
            false
        }
    }
}