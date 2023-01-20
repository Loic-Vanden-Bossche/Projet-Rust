#![allow(non_snake_case)]

mod types;
mod function;
mod challenges;
mod tui;

use std::io::Stdout;
use ::tui::backend::CrosstermBackend;
use ::tui::Terminal;
use crate::function::args::parse_args;
use crate::function::game::game;
use crate::tui::error::UIError;
use crate::types::end::EndOfGame;
use crate::tui::input::InputMode;
use crate::tui::menu::{MenuItem};
use crate::tui::ui::ui;
use crate::types::challenge::Challenge;
use crate::types::player::PublicLeaderBoard;

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

fn main() {
	let (name, port, debug, host, no_ui) = parse_args();
	if no_ui{
		game(host, port, name, debug);
	}else{
		ui(debug);
	};
}