use std::ops::Add;
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Paragraph};
use crate::State;

pub fn basic_block<'a>(title: String) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Rgb(226, 135, 67)))
        .title(title)
        .border_type(BorderType::Rounded)
}

pub fn make_status<'a>(connected: bool) -> Paragraph<'a>{
    let (statut, color) = if connected {
        ("Connecté", Color::Blue)
    }else{
        ("Non connecté", Color::Gray)
    };
    Paragraph::new(Spans(vec![
        Span::styled("Statut : ", Style::default().fg(Color::White)),
        Span::styled(statut, Style::default().fg(color))
    ]))
        .alignment(Alignment::Left)
        .block(basic_block("Statut".to_string()))
}

pub fn make_copright<'a>() -> Paragraph<'a>{
    Paragraph::new("patate - Groupe 1 4AL2 - SOARES Enzo - TURBIEZ Denis - VANDEN BOSSCHE Loïc - 2022/2023")
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center)
        .block(basic_block("Copyright".to_string()))
}