use std::io::Stdout;
use log::{debug, error};
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Paragraph};
use crate::function::round::get_player;
use crate::tui::error::UIError;
use crate::tui::ui::State;

pub fn basic_block<'a>(title: String) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Rgb(226, 135, 67)))
        .title(title)
        .border_type(BorderType::Rounded)
}

pub fn render_status<'a>(state: &mut State, chunk: Rect, rect: &mut Frame<CrosstermBackend<Stdout>>){
    let (statut, color) = if state.connected {
        ("Connecté", Color::Blue)
    }else{
        ("Non connecté", Color::Gray)
    };
    let board = match state.summary.as_ref(){
        Some(val) => {
            val.PublicLeaderBoard.clone()
        }
        None => {
            vec![]
        }
    };
    let (game_started, color_game) = if state.connected && state.summary.is_some() {
        if state.eog.is_none() {
            ("En cours", Color::Green)
        }else{
            ("Terminée", Color::Blue)
        }
    }else {
        ("En attente", Color::Gray)
    };
    let (game_res, color_res) = if state.eog.is_none() {
        ("En attente", Color::Gray)
    }else {
        let top1 = match get_player(&board, &state.name, false){
            Some(player) => {
                debug!("Successfully retrieve winner");
                player
            }
            None => {
                error!("Cannot retrieve winner");
                state.error = Some(UIError::FatalError);
                return;
            }
        };
        if top1.name.eq(&state.name.clone()){
            ("Victoire", Color::Green)
        }else {
            ("Défaite", Color::Red)
        }
    };
    let (error, color_error, modifier) = match state.error.as_ref(){
        Some(error) => {
            let message = match error {
                UIError::ConnectError => {
                    "Erreur lors de la connexion"
                }
                UIError::FatalError => {
                    "Erreur fatale"
                }
            };
            (message, Color::Red, Modifier::SLOW_BLINK)
        }
        None => ("Rien à signaler", Color::LightGreen, Modifier::empty())
    };
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25)
        ].as_ref())
        .split(chunk);
    let statut = Paragraph::new(Spans(vec![
        Span::styled("Statut : ", Style::default().fg(Color::White)),
        Span::styled(statut, Style::default().fg(color))
    ]))
        .alignment(Alignment::Left)
        .block(basic_block("Statut".to_string()));
    let game = Paragraph::new(Spans(vec![
        Span::styled("Partie : ", Style::default().fg(Color::White)),
        Span::styled(game_started, Style::default().fg(color_game))
    ]))
        .alignment(Alignment::Left)
        .block(basic_block("Partie".to_string()));
    let res = Paragraph::new(Spans(vec![
        Span::styled("Résultat : ", Style::default().fg(Color::White)),
        Span::styled(game_res, Style::default().fg(color_res))
    ]))
        .alignment(Alignment::Left)
        .block(basic_block("Résultat".to_string()));
    let err = Paragraph::new(Spans(vec![
        Span::styled("Erreur : ", Style::default().fg(Color::Red)),
        Span::styled(error, Style::default().fg(color_error).add_modifier(modifier))
    ]))
        .alignment(Alignment::Left)
        .block(basic_block("Erreur".to_string()));
    rect.render_widget(statut, chunks[0]);
    rect.render_widget(game, chunks[1]);
    rect.render_widget(res, chunks[2]);
    rect.render_widget(err, chunks[3]);
}

pub fn make_copright<'a>() -> Paragraph<'a>{
    Paragraph::new("patate - Groupe 1 4AL2 - SOARES Enzo - TURBIEZ Denis - VANDEN BOSSCHE Loïc - 2022/2023")
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center)
        .block(basic_block("Copyright".to_string()))
}