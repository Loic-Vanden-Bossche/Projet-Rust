use std::{io};
use std::io::Stdout;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use log::{error};
use tui::backend::CrosstermBackend;
use tui::{Terminal};
use tui::layout::{Constraint, Direction, Layout};
use crate::tui::block::make_copright;
use crate::tui::menu::{make_tabs, MenuItem, render_active_menu};

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

pub fn draw(term: &mut Terminal<CrosstermBackend<Stdout>>, menu_titles: &Vec<&str>, active_menu_item: MenuItem, input: &String){
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
        rect.render_widget(make_tabs(&menu_titles, active_menu_item), chunks[0]);
        render_active_menu(active_menu_item, rect, chunks[1], &input)
    }).expect("Pannik");
}

pub fn clear(term: &mut Terminal<CrosstermBackend<Stdout>>){
    if let Err(_) = term.clear() {
        error!("Error while clearing terminal");
    }
}