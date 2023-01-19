#![allow(non_snake_case)]

extern crate core;

mod types;
mod function;
mod challenges;
mod tui;

use std::io::Stdout;
use std::net::TcpStream;
use std::ops::Add;
use std::sync::mpsc;
use ::tui::backend::CrosstermBackend;
use ::tui::Terminal;
use log::{info, error, debug};
use simplelog::{ColorChoice, Config, TerminalMode};
use crate::function::args::parse_args;
use crate::function::connect::connect;
use crate::function::round::{get_player, round};
use crate::tui::error::UIError;
use crate::tui::term::{clear, draw, get_term};
use crate::types::end::EndOfGame;
use crate::types::error::{RoundErrorReason};
use crate::tui::event::{event_loop, receive_event};
use crate::tui::input::InputMode;
use crate::tui::menu::{MenuItem};

fn make_url(host: Option<String>, port: u32) -> String{
	match host {
		Some(host) => { host }
		None => { "localhost".to_string() }
	}.add(":").add(port.to_string().as_str())
}

pub struct State{
	stream: Option<TcpStream>,
	connected: bool,
	name: String,
	input_mode: InputMode,
	active_menu: MenuItem,
	term: Terminal<CrosstermBackend<Stdout>>,
	error: Option<UIError>
}

fn ui(){
	let (tx, rx) = mpsc::channel();
	event_loop(tx);
	let mut state = State{
		connected: false,
		name: "".to_string(),
		input_mode: InputMode::User,
		active_menu: MenuItem::Intro,
		stream: None,
		term: get_term(),
		error: None
	};
	let menu_titles = vec!["Intro", "Résumé", "Actuel", "Split", "Quitter"];

	clear(&mut state.term);
	loop {
		draw(&mut state, &menu_titles);
		if !receive_event(&rx, &mut state){
			break;
		}
	}
}



fn main() {
	let (name, port, debug, host) = if let Some(val) = parse_args(){
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