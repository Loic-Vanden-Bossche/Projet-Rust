use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Tabs};
use crate::State;
use crate::tui::block::{basic_block};

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

pub fn render_active_menu(active_menu_item: MenuItem, rect: &mut Frame<CrosstermBackend<Stdout>>, chunk: Rect, input: &String){
    match active_menu_item {
        MenuItem::Intro => rect.render_widget(make_intro(input.clone()), chunk),
        MenuItem::Summary => rect.render_widget(make_summary(), chunk),
        MenuItem::CurrentChallenge => rect.render_widget(make_current(), chunk),
        MenuItem::Split => render_split(chunk, rect),
        _ => {}
    };
}

pub fn make_intro<'a>(input: String) -> Paragraph<'a> {
    let home = Paragraph::new(vec![
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
    ])
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White))
        .block(basic_block("Intro".to_string()));
    home
}