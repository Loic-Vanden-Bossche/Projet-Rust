#![allow(non_snake_case)]

extern crate core;

mod types;
mod function;
mod challenges;
mod tui;

use std::ops::Add;
use std::sync::mpsc;
use crossterm::event::{KeyCode};
use log::{info, error};
use simplelog::{ColorChoice, Config, debug, TerminalMode};
use crate::function::args::parse_args;
use crate::function::connect::connect;
use crate::function::round::{get_player, round};
use crate::tui::term::{clear, close_term, draw, get_term};
use crate::types::end::EndOfGame;
use crate::types::error::{RoundErrorReason};
use crate::tui::event::{Event, event_loop, receive_event};
use crate::tui::input::InputMode;
use crate::tui::menu::{MenuItem};

fn make_url(host: Option<String>, port: u32) -> String{
	match host {
		Some(host) => { host }
		None => { "localhost".to_string() }
	}.add(":").add(port.to_string().as_str())
}

fn main(){
	let mut input_mode = InputMode::User;
	let mut input: String = "".to_string();
	let mut active_menu_item = MenuItem::Intro;
	let menu_titles = vec!["Intro", "Résumé", "Actuel", "Split", "Quitter"];
	let (tx, rx) = mpsc::channel();
	event_loop(tx);
	let mut term = get_term();
	clear(&mut term);
	loop {
		draw(&mut term, &menu_titles, active_menu_item, &input);
		if !receive_event(&rx, &mut input_mode, &mut active_menu_item, &mut input, &mut term){
			break;
		}
	}
}



fn main2() {
	let (name, port, debug, host) = parse_args();
	match simplelog::TermLogger::init(debug, Config::default(), TerminalMode::Mixed, ColorChoice::Always) {
		Ok(_) => { debug!("Logger loaded") }
		Err(err) => {
			println!("Error on loading logger: {err}")
		}
	}
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