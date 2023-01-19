use std::{io, thread};
use std::io::Stdout;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen};
use log::{debug, error};
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::{Frame, Terminal};
use tui::widgets::{Block, Borders, BorderType, Paragraph};
use crossterm::event::Event as CEvent;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use crate::tui::block::make_copright;
use crate::tui::menu::{make_tabs, MenuItem, render_active_menu};

pub fn get_term() -> Terminal<CrosstermBackend<Stdout>>{
    enable_raw_mode().expect("Raw mode");
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend).expect("NIKKKK");
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