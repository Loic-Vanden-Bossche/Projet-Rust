#![allow(non_snake_case)]

extern crate core;

mod types;
mod function;
mod challenges;
mod tui;

use std::{io, thread};
use std::io::Stdout;
use std::ops::Add;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use ::tui::backend::CrosstermBackend;
use ::tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ::tui::style::{Color, Modifier, Style};
use ::tui::{Frame, Terminal};
use ::tui::text::{Span, Spans};
use ::tui::widgets::{Block, Borders, BorderType, Paragraph, Tabs};
use crossterm::event;
use crossterm::event::{Event as CEvent, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use log::{info, error, LevelFilter};
use simplelog::{ColorChoice, Config, debug, TerminalMode};
use crate::function::args::parse_args;
use crate::function::connect::connect;
use crate::function::round::{get_player, round};
use crate::tui::term::{basic_block, close_term, get_term, make_copright, make_current, make_intro, make_summary, render_split};
use crate::types::end::EndOfGame;
use crate::types::error::{RoundErrorReason};
use crate::tui::event::{Event, event_loop};
use crate::tui::input::InputMode;
use crate::tui::menu::{make_menu, MenuItem};

fn make_url(host: Option<String>, port: u32) -> String{
	match host {
		Some(host) => { host }
		None => { "localhost".to_string() }
	}.add(":").add(port.to_string().as_str())
}

fn main(){
	let mut input_mode = InputMode::User;
	let mut input: String = "".to_string();
	let (tx, rx) = mpsc::channel();
	event_loop(tx);
	let mut term = get_term();
	term.clear().expect("Clear");
	let menu_titles = vec!["Intro", "Résumé", "Actuel", "Split", "Quitter"];
	let mut active_menu_item = MenuItem::Intro;
	loop {
		term.draw(|rect| {
			let size = rect.size();
			let chunks = Layout::default()
				.direction(Direction::Vertical)
				.margin(1)
				.constraints(
					[
						Constraint::Length(3),
						Constraint::Min(2),
						Constraint::Length(3)
					].as_ref(),
				)
				.split(size);
			rect.render_widget(make_copright(), chunks[2]);
			let menu = make_menu(&menu_titles);
			let tabs = Tabs::new(menu)
				.select(active_menu_item.into())
				.block(basic_block("Menu".to_string()))
				.style(Style::default().fg(Color::White))
				.highlight_style(Style::default().fg(Color::Yellow))
				.divider(Span::raw("|"));
			rect.render_widget(tabs, chunks[0]);
			match active_menu_item {
				MenuItem::Intro => rect.render_widget(make_intro(input.clone()), chunks[1]),
				MenuItem::Summary => rect.render_widget(make_summary(), chunks[1]),
				MenuItem::CurrentChallenge => rect.render_widget(make_current(), chunks[1]),
				MenuItem::Split => render_split(chunks[1], rect),
				_ => {}
			};
		}).expect("Pannik");
		match rx.recv().unwrap() {
			Event::Input(event) => {
				match input_mode {
					InputMode::Normal => match event.code {
						KeyCode::Char('q') => {
							close_term(&mut term);
							break;
						}
						KeyCode::Char('i') => {
							active_menu_item = MenuItem::Intro;
							input = "".to_string();
							input_mode = InputMode::User;
						},
						KeyCode::Char('r') => active_menu_item = MenuItem::Summary,
						KeyCode::Char('a') => active_menu_item = MenuItem::CurrentChallenge,
						KeyCode::Char('s') => active_menu_item = MenuItem::Split,
						_ => {}
					}
					InputMode::User => match event.code {
						KeyCode::Enter => {
							input_mode = InputMode::Normal;
							active_menu_item = MenuItem::Summary;
						}
						KeyCode::Char(c) => input.push(c),
						KeyCode::Backspace => {
							input.pop();
						},
						KeyCode::Esc => {
							close_term(&mut term);
							break;
						}
						_ => {}
					}
				}
			},
			Event::Tick => {}
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