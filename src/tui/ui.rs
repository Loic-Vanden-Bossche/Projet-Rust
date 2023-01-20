use std::fs::File;
use std::sync::mpsc;
use log::{debug, LevelFilter};
use simplelog::Config;
use crate::State;
use crate::tui::event::{event_loop, receive_event};
use crate::tui::game::game;
use crate::tui::input::InputMode;
use crate::tui::menu::MenuItem;
use crate::tui::term::{clear, draw, get_term};

pub fn ui(debug: LevelFilter){
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
    game(rS, tx);
    clear(&mut term);
    match File::create("./log"){
        Ok(f) => match simplelog::WriteLogger::init(debug, Config::default(), f){
            Ok(_) => {
                debug!("File logger successfully loaded");
            }
            Err(e) => {
                println!("Cannot load logger : {e}");
            }
        }
        Err(e) => {
            println!("Error : Can't open log file\nReason {e}");
        }
    }


    loop {
        draw(&mut state, &menu_titles, &mut term);
        if !receive_event(&rx, &sS, &mut state, &mut term){
            break;
        }
    }
}