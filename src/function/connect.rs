use std::net::TcpStream;
use serde_json::to_string;
use crate::function::stream::{read_from_stream, write_to_stream};
use crate::types::error::{Error};
use crate::types::subscribe::{Name, Subscribe, SubscribeError, SubscribeResult, SubscribeResultEnum};
use crate::types::welcome::Welcome;

fn get_stream(ip: String) -> Result<TcpStream, Error>{
    return match TcpStream::connect(ip) {
        Ok(s) => {
            Ok(s)
        }
        Err(_) => {
            println!("Failed to connect");
            Err(Error{coucou: 4})
        }
    };
}

fn hello(stream: &TcpStream) -> bool {
    write_to_stream(&stream, "\"Hello\"".to_string());
    return match read_from_stream(&stream){
        Ok(val) => {
            let val: Welcome = val;
            if val.Welcome.version == 1 {
                true
            }else{
                println!("Unsupported version");
                false
            }
        }
        Err(_) => {
            println!("Error while hello");
            false
        }
    }
}

fn subscribe(stream: &TcpStream, name: String) -> Result<i32, Error> {
    let sub = Subscribe{Subscribe: Name{ name }};
    write_to_stream(&stream, to_string(&sub).expect("Impossible"));
    let sub_res: SubscribeResult = match read_from_stream(&stream) {
        Ok(val) => {
            val
        }
        Err(_) => {
            return Err(Error{coucou: 5})
        }
    };
    match sub_res.SubscribeResult {
        SubscribeResultEnum::Ok => {
            Ok(1)
        }
        SubscribeResultEnum::Err(err) => {
            match err {
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

pub fn connect(ip: String, name: String) -> Result<TcpStream, Error>{
    let stream = match get_stream(ip) {
        Ok(s) => {
            s
        }
        Err(err) => {
            println!("Failed to connect");
            return Err(err);
        }
    };
    if hello(&stream){
        match subscribe(&stream, name) {
            Ok(_) => {
                Ok(stream)
            }
            Err(err) => {
                println!("Error while subscribing");
                Err(err)
            }
        }
    }else{
        Err(Error{coucou: 5})
    }
}