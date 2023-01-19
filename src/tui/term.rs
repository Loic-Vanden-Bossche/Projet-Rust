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

pub fn get_term() -> Terminal<CrosstermBackend<Stdout>>{
    enable_raw_mode().expect("Raw mode");
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend).expect("NIKKKK");
    return term;
}

pub fn close_term(term: &mut Terminal<CrosstermBackend<Stdout>>){
    if let Err(_) = term.clear() {
        error!("Error clearing terminal");
    }
    if let Err(_) = disable_raw_mode() {
        error!("Error disabling raw mode");
    }
    if let Err(_) = term.show_cursor() {
        error!("Error showing cursor in terminal");
    }
}

pub fn basic_block<'a>(title: String) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Rgb(226, 135, 67)))
        .title(title)
        .border_type(BorderType::Rounded)
}

pub fn make_summary<'a>() -> Paragraph<'a> {
    Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Summary")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Coucou")])
    ])
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Green))
        .block(basic_block("Résumé".to_string()))
}

pub fn make_current<'a>() -> Paragraph<'a> {
    Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Current")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Coucou")])
    ])
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Green))
        .block(basic_block("Actuel".to_string()))
}

pub fn render_split(chunk: Rect, rect: &mut Frame<'_, CrosstermBackend<Stdout>>){
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ].as_ref())
        .split(chunk);
    rect.render_widget(make_summary(), chunks[0]);
    rect.render_widget(make_current(), chunks[1]);
}

pub fn make_intro<'a>(input: String) -> Paragraph<'a> {
    let home = Paragraph::new((vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Bienvenue")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("à la")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled("patate", Style::default().fg(Color::LightBlue))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Entre ton nom pour te connecter au serveur")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(input)])
    ]))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White))
        .block(basic_block("Intro".to_string()));
    home
}

pub fn make_copright<'a>() -> Paragraph<'a>{
    Paragraph::new("patate - Groupe 1 4AL2 - SOARES Enzo - TURBIEZ Denis - VANDEN BOSSCHE Loïc - 2022/2023")
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center)
        .block(basic_block("Copyright".to_string()))
}