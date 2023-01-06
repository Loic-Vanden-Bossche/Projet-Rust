use std::net::TcpStream;
use log::{error, trace, warn};
use serde_json::to_string;
use simplelog::debug;
use crate::function::stream::{read_from_stream, write_to_stream};
use crate::types::subscribe::{Name, Subscribe, SubscribeError, SubscribeResult, SubscribeResultEnum};
use crate::types::welcome::Welcome;

fn get_stream(ip: String) -> Option<TcpStream>{
    match TcpStream::connect(ip) {
        Ok(s) => {
            debug!("Socket successfully connected");
            Some(s)
        }
        Err(e) => {
            debug!("Socket failed to connect, {e}");
            None
        }
    }
}

fn hello(stream: &TcpStream) -> bool {
    if !write_to_stream(&stream, "\"Hello\"".to_string()){
        debug!("Error on \"Hello\"");
        return false;
    }
    match read_from_stream(&stream){
        Ok(val) => {
            let val: Welcome = val;
            if val.Welcome.version == 1 {
                true
            }else{
                error!("Unsupported version");
                false
            }
        }
        Err(_) => {
            error!("Error on Welcome");
            false
        }
    }
}

fn subscribe(stream: &TcpStream, name: String) -> bool {
    match to_string(&Subscribe{Subscribe: Name{ name }}) {
        Ok(val) => {
            if write_to_stream(stream, val) {
                debug!("Successfully send subscribe request")
            }else {
                error!("Error on sending subscribe request")
            }
        }
        Err(e) => {
            trace!("Error on stringify data: {e}");
            return false;
        }
    };
    let sub_res: SubscribeResult = match read_from_stream(&stream) {
        Ok(val) => {
            debug!("Retrieve subscribe result");
            val
        }
        Err(_) => {
            error!("Error on retrieving subscribe result");
            return false;
        }
    };
    match sub_res.SubscribeResult {
        SubscribeResultEnum::Ok => {
            debug!("Successfully subscribed");
            true
        }
        SubscribeResultEnum::Err(err) => {
            match err {
                SubscribeError::InvalidName => {
                    error!("Invalid Name");
                    false
                }
                SubscribeError::AlreadyRegistered => {
                    warn!("Already registered");
                    false
                }
            }
        }
    }
}

pub fn connect(ip: String, name: String) -> Option<TcpStream>{
    let stream = match get_stream(ip) {
        Some(s) => {
            debug!("Socket created");
            s
        }
        None => {
            error!("Failed to connect");
            return None;
        }
    };
    if hello(&stream){
        debug!("Successfully hello");
        if subscribe(&stream, name) {
            debug!("Successfully subscribed");
            Some(stream)
        } else {
            error!("Error on subscribe phase");
            None
        }
    }else{
        error!("Error on hello phase");
        None
    }
}