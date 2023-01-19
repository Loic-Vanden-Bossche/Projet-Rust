use std::{io};
use std::io::Stdout;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use log::{error};
use tui::backend::CrosstermBackend;
use tui::{Terminal};
use tui::layout::{Constraint, Direction, Layout};
use crate::State;
use crate::tui::block::{make_copright, make_status};
use crate::tui::menu::{make_tabs, render_active_menu};

pub fn get_term() -> Terminal<CrosstermBackend<Stdout>>{
    enable_raw_mode().expect("Raw mode");
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let term = Terminal::new(backend).expect("NIKKKK");
    return term;
}

pub fn close_term(term: &mut Terminal<CrosstermBackend<Stdout>>){
    clear(term);
    if let Err(_) = disable_raw_mode() {
        error!("Error disabling raw mode");
    }
    if let Err(_) = term.show_cursor() {
        error!("Error showing cursor in terminal");
    }
}

pub fn draw(state: &mut State, menu_titles: &Vec<&str>){
    state.term.draw(|rect| {
        let size = rect.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3)
                ].as_ref(),
            )
            .split(size);
        rect.render_widget(make_copright(), chunks[3]);
        rect.render_widget(make_tabs(&menu_titles, state.active_menu), chunks[0]);
        rect.render_widget(make_status(state.connected), chunks[1]);
        render_active_menu(state.active_menu, rect, chunks[2], &state.name);
    }).expect("Pannik");
}

pub fn clear(term: &mut Terminal<CrosstermBackend<Stdout>>){
    if let Err(_) = term.clear() {
        error!("Error while clearing terminal");
    }
}