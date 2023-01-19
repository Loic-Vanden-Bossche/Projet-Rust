#![allow(non_snake_case)]

extern crate core;

mod types;
mod function;
mod challenges;
mod tui;

use std::fs::File;
use std::io::Stdout;
use std::net::TcpStream;
use std::ops::Add;
use std::sync::mpsc;
use std::thread;
use ::tui::backend::CrosstermBackend;
use ::tui::Terminal;
use log::{info, error, debug, LevelFilter};
use simplelog::{ColorChoice, Config, SimpleLogger, TerminalMode};
use crate::function::args::parse_args;
use crate::function::connect::connect;
use crate::function::round::{get_challenge_input, get_player, respond_challenge, round, start_round};
use crate::tui::error::UIError;
use crate::tui::term::{clear, draw, get_term};
use crate::types::end::EndOfGame;
use crate::types::error::{ChallengeError, RoundErrorReason, RoundStartErrorEnum};
use crate::tui::event::{Event, event_loop, GameEvent, receive_event};
use crate::tui::input::InputMode;
use crate::tui::menu::{MenuItem};
use crate::types::challenge::Challenge;
use crate::types::player::PublicLeaderBoard;
use crate::types::round::RoundSummary;

fn make_url(host: Option<String>, port: u32) -> String{
	match host {
		Some(host) => { host }
		None => { "localhost".to_string() }
	}.add(":").add(port.to_string().as_str())
}

#[derive(Clone)]
pub struct State{
	connected: bool,
	name: String,
	input_mode: InputMode,
	active_menu: MenuItem,
	error: Option<UIError>,
	summary: Option<PublicLeaderBoard>,
	eog: Option<EndOfGame>,
	current: Option<Challenge>
}

type Term = Terminal<CrosstermBackend<Stdout>>;

fn ui(){
	let (tx, rx) = mpsc::channel();
	let (sS, rS) = mpsc::channel();
	event_loop(tx.clone());
	let mut term = get_term();
	let mut state = State{
		connected: false,
		name: "".to_string(),
		input_mode: InputMode::User,
		active_menu: MenuItem::Intro,
		error: None,
		summary: None,
		eog: None,
		current: None
	};
	let menu_titles = vec!["Intro", "Résumé", "Actuel", "Split", "Quitter"];

	clear(&mut term);
	let f = File::create("./log").unwrap();
	simplelog::WriteLogger::init(LevelFilter::Debug, Config::default(), f);
	thread::spawn( move ||{
		let (stream, name): (TcpStream, String) = rS.recv().unwrap();
		debug!("Stream received");
		loop {
			let plb = match start_round(&stream) {
				Ok(val) => {
					tx.send(Event::Game(GameEvent::PublicLeaderBoard(val.clone())));
					val
				}
				Err(e) => {
					match e.reason {
						RoundStartErrorEnum::EndOfGame(eog) => {
							tx.send(Event::Game(GameEvent::EndOfGame(eog)));
							break;
						}
						RoundStartErrorEnum::ReadError => {
							tx.send(Event::Game(GameEvent::Error(UIError::FatalError)));
							return;
						}
					}
				}
			};
			let top1 = get_player(&plb.PublicLeaderBoard, &name, true).unwrap();
			loop {
				let challenge: Challenge = match get_challenge_input(&stream){
					Ok(val) => {
						debug!("Get challenge input");
						tx.send(Event::Game(GameEvent::ChallengeInput(val.clone())));
						val
					},
					Err(e) => match e {
						ChallengeError::EndOfRound(val) => {
							debug!("End of round");
							tx.send(Event::Game(GameEvent::EndOfRound(val)));
							break;
						}
						ChallengeError::ChallengeInput => {
							error!("Invalid data receive at start of challenge");
							tx.send(Event::Game(GameEvent::Error(UIError::FatalError)));
							break;
						}
					}
				};
				respond_challenge(&stream, top1, challenge);
			}
		}
	});

	loop {
		draw(&mut state, &menu_titles, &mut term);
		if !receive_event(&rx, &sS, &mut state, &mut term){
			break;
		}
	}
}



fn main() {
	let (name, port, _, host) = if let Some(val) = parse_args(){
		match simplelog::TermLogger::init(val.2, Config::default(), TerminalMode::Mixed, ColorChoice::Always) {
			Ok(_) => { debug!("Logger loaded") }
			Err(err) => {
				println!("Error on loading logger: {err}")
			}
		}
		info!("No UI");
		let name = if let Some(val) = val.0{
			val
		}else{
			error!("Name required without UI");
			return;
		};
		(name, val.1, val.2, val.3)
	}else{
		ui();
		return;
	};
	let stream = match connect(make_url(host, port), &name) {
		Some(s) => {
			info!("Connected");
			s
		}
		None => {
			error!("Error while connecting");
			return;
		}
	};
	let end: EndOfGame;
	loop {
		match round(&stream, &name) {
			Ok( sum ) => {
				info!("Challenge completed: {}", sum.RoundSummary.challenge)
			}
			Err(e) => {
				match e.reason {
					RoundErrorReason::EndOfGame(eog) => {
						end = eog;
						break;
					}
					RoundErrorReason::StartError => {
						error!("Error starting a round");
						return;
					}
					RoundErrorReason::LeaderBoardError => {
						error!("Error getting leader board");
						return;
					}
					RoundErrorReason::EndError => {
						error!("Error ending a round");
						return;
					}
				}
			}
		}
	}
	let top1 = match get_player(&end.EndOfGame.leader_board, &name, false) {
		Some(val) => { val }
		None => {
			error!("No player on leaderboard");
			return;
		}
	};
	info!("Player {} win with {} point! GG", top1.name, top1.score);
}