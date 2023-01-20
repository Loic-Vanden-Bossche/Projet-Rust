#![allow(non_snake_case)]

mod types;
mod function;
mod challenges;
mod tui;

use crate::function::args::parse_args;
use crate::function::game::game;
use crate::tui::ui::ui;

fn main() {
	let (name, port, debug, host, no_ui) = parse_args();
	if no_ui{
		game(host, port, name, debug);
	}else{
		ui(host, port, debug);
	};
}