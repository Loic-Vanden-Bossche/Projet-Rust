use std::{io};
use std::io::Stdout;
use std::process::exit;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use log::{debug, error, trace};
use tui::backend::CrosstermBackend;
use tui::{Terminal};
use tui::layout::{Constraint, Direction, Layout};
use crate::{State, Term};
use crate::tui::block::{make_copright, render_status};
use crate::tui::menu::{make_tabs, render_active_menu};

pub fn get_term() -> Term{
    match enable_raw_mode() {
        Ok(_) => {
            debug!("Successfully enable raw mode")
        }
        Err(err) => {
            error!("Error enabling raw mode: {err}");
        }
    }
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    match Terminal::new(backend) {
        Ok(term) => {
            debug!("Terminal created");
            term
        }
        Err(err) => {
            error!("Cannot create terminal: {err}");
            exit(-1);
        }
    }
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

pub fn draw(state: &mut State, menu_titles: &Vec<&str>, term: &mut Term){
    match term.draw(|rect| {
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
        render_status(state, chunks[1], rect);
        render_active_menu(state, rect, chunks[2]);
    }) {
        Ok(_) => {
            trace!("Draw Ok");
        }
        Err(err) => {
            error!("Error in drawing: {err}");
        }
    };
}

pub fn clear(term: &mut Terminal<CrosstermBackend<Stdout>>){
    if let Err(_) = term.clear() {
        error!("Error while clearing terminal");
    }
}