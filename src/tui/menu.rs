use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Tabs};
use crate::tui::block::{basic_block};
use crate::tui::ui::State;
use crate::types::challenge::{Challenge, ChallengeEnum};
use crate::types::player::PublicLeaderBoard;

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Intro,
    Summary,
    CurrentChallenge,
    Split,
    Quit
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Intro => 0,
            MenuItem::Summary => 1,
            MenuItem::CurrentChallenge => 2,
            MenuItem::Split => 3,
            MenuItem::Quit => 4
        }
    }
}

pub fn make_tabs<'a>(menu_titles: &Vec<&'a str>, active_menu_item: MenuItem) -> Tabs<'a>{
    let menu = make_menu(&menu_titles);
    Tabs::new(menu)
        .select(active_menu_item.into())
        .block(basic_block("Menu".to_string()))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"))
}

pub fn make_menu<'a>(menu_titles: &Vec<&'a str>) -> Vec<Spans<'a>>{
    menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect()
}

pub fn make_summary<'a>(plb: &Option<PublicLeaderBoard>) -> Paragraph<'a> {
    let mut data: Vec<Spans> = vec![];
    if let Some(plb) = plb {
        let mut sorted = plb.PublicLeaderBoard.clone();
        sorted.sort_by(|a, b| b.score.cmp(&a.score));
        for p in sorted {
            let (name_color, score_color) = if p.is_active {
                (Color::Green, Color::White)
            }else {
                (Color::Red, Color::DarkGray)
            };
            data.push(
                Spans::from(vec![
                    Span::styled(p.name, Style::default().fg(name_color)),
                    Span::styled(" : ", Style::default().fg(Color::White)),
                    Span::styled(p.score.to_string(), Style::default().fg(score_color))
                ])
            );
        }
    }
    Paragraph::new(data)
        .alignment(Alignment::Left)
        .block(basic_block("Résumé".to_string()))
}

pub fn make_current(challenge: &Option<Challenge>) -> Paragraph {
    let mut data: Vec<Spans> = vec![];
    if let Some(challenge) = challenge {
        match &challenge.Challenge {
            ChallengeEnum::MD5HashCash(val) => {
                data.push(Spans::from(vec![Span::raw("Complexity : "), Span::raw(val.complexity.to_string())]));
                data.push(Spans::from(vec![Span::raw("Seed : "), Span::raw(val.message.clone())]));
            }
            ChallengeEnum::MonstrousMaze(val) => {
                for line in val.grid.lines() {
                    data.push(Spans::from(Span::styled(line.clone(), Style::default().fg(Color::Magenta))));
                }
            }
        }
    }
    Paragraph::new(data)
        .alignment(Alignment::Left)
        .block(basic_block("Actuel".to_string()))
}

pub fn render_split(chunk: Rect, rect: &mut Frame<'_, CrosstermBackend<Stdout>>, plb: &Option<PublicLeaderBoard>, challenge: &Option<Challenge>){
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ].as_ref())
        .split(chunk);
    rect.render_widget(make_summary(plb), chunks[0]);
    rect.render_widget(make_current(challenge), chunks[1]);
}

pub fn render_active_menu(state: &State, rect: &mut Frame<CrosstermBackend<Stdout>>, chunk: Rect){
    match state.active_menu {
        MenuItem::Intro => rect.render_widget(make_intro(state.name.clone()), chunk),
        MenuItem::Summary => rect.render_widget(make_summary(&state.summary), chunk),
        MenuItem::CurrentChallenge => rect.render_widget(make_current(&state.current), chunk),
        MenuItem::Split => render_split(chunk, rect, &state.summary, &state.current),
        _ => {}
    };
}

pub fn make_intro<'a>(input: String) -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled("Je suis une patate", Style::default().fg(Color::LightBlue))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Je génère 1.1 volt d'électricité")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Entre ton nom pour te connecter au serveur")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(input)])
    ])
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White))
        .block(basic_block("Intro".to_string()));
    home
}