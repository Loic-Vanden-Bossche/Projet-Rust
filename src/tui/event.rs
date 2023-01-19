use std::io::Stdout;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};
use crossterm::event;
use crossterm::event::{Event as CEvent, KeyCode, KeyEvent};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use crate::tui::input::InputMode;
use crate::tui::menu::MenuItem;
use crate::tui::term::close_term;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub fn event_loop(tx: Sender<Event<KeyEvent>>){
    let tick_rate = Duration::from_millis(100);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }
            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now()
                }
            }
        }
    });
}

pub fn receive_event(rx: &Receiver<Event<KeyEvent>>, input_mode: &mut InputMode, active_menu_item: &mut MenuItem, input: &mut String, term: &mut Terminal<CrosstermBackend<Stdout>>) -> bool {
    match rx.recv().unwrap() {
        Event::Input(event) => {
            match input_mode {
                InputMode::Normal => match event.code {
                    KeyCode::Char('q') => {
                        close_term(term);
                        return false;
                    }
                    KeyCode::Char('i') => {
                        *active_menu_item = MenuItem::Intro;
                        *input = "".to_string();
                        *input_mode = InputMode::User;
                    },
                    KeyCode::Char('r') => *active_menu_item = MenuItem::Summary,
                    KeyCode::Char('a') => *active_menu_item = MenuItem::CurrentChallenge,
                    KeyCode::Char('s') => *active_menu_item = MenuItem::Split,
                    _ => {}
                }
                InputMode::User => match event.code {
                    KeyCode::Enter => {
                        *input_mode = InputMode::Normal;
                        *active_menu_item = MenuItem::Summary;
                    }
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => {
                        input.pop();
                    },
                    KeyCode::Esc => {
                        close_term(term);
                        return false;
                    }
                    _ => {}
                }
            }
        },
        Event::Tick => {}
    };
    return true;
}