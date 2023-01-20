use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crossterm::event::KeyEvent;
use log::{debug, error};
use crate::function::round::{get_challenge_input, get_player, respond_challenge, start_round};
use crate::tui::error::UIError;
use crate::tui::event::{Event, GameEvent, send};
use crate::types::challenge::Challenge;
use crate::types::error::{ChallengeError, RoundStartErrorEnum};

pub fn game(rS: Receiver<(TcpStream, String)>, tx: Sender<Event<KeyEvent>>){
    thread::spawn( move ||{
        let (stream, name): (TcpStream, String) = match rS.recv(){
            Ok(val) => {
                debug!("Successfully received stream and name in threaad [game]");
                val
            }
            Err(err) => {
                error!("Error receiving stream and name in thread [game]: {err}");
                return;
            }
        };
        loop {
            let plb = match start_round(&stream) {
                Ok(val) => {
                    send(&tx, Event::Game(GameEvent::PublicLeaderBoard(val.clone())));
                    val
                }
                Err(e) => {
                    match e.reason {
                        RoundStartErrorEnum::EndOfGame(eog) => {
                            send(&tx, Event::Game(GameEvent::EndOfGame(eog)));
                            break;
                        }
                        RoundStartErrorEnum::ReadError => {
                            send(&tx, Event::Game(GameEvent::Error(UIError::FatalError)));
                            return;
                        }
                    }
                }
            };
            let top1 = match get_player(&plb.PublicLeaderBoard, &name, true) {
                Some(player) => {
                    debug!("Retrieve next player");
                    player
                }
                None => {
                    error!("Error retrieving next player");
                    send(&tx, Event::Game(GameEvent::Error(UIError::FatalError)));
                    return;
                }
            };
            loop {
                let challenge: Challenge = match get_challenge_input(&stream){
                    Ok(val) => {
                        debug!("Get challenge input");
                        send(&tx, Event::Game(GameEvent::ChallengeInput(val.clone())));
                        val
                    },
                    Err(e) => match e {
                        ChallengeError::EndOfRound(val) => {
                            debug!("End of round");
                            send(&tx, Event::Game(GameEvent::EndOfRound(val)));
                            break;
                        }
                        ChallengeError::ChallengeInput => {
                            error!("Invalid data receive at start of challenge");
                            send(&tx, Event::Game(GameEvent::Error(UIError::FatalError)));
                            break;
                        }
                    }
                };
                respond_challenge(&stream, top1, challenge);
            }
        }
    });
}