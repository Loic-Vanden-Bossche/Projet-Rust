use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};

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