use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};
use crossterm::event;
use crossterm::event::{Event as CEvent, KeyCode, KeyEvent};
use crate::function::connect::connect;
use crate::State;
use crate::tui::error::UIError;
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

pub fn receive_event(rx: &Receiver<Event<KeyEvent>>, state: &mut State) -> bool {
    match rx.recv().unwrap() {
        Event::Input(event) => {
            match state.input_mode {
                InputMode::Normal => match event.code {
                    KeyCode::Char('q') => {
                        close_term(&mut state.term);
                        return false;
                    }
                    KeyCode::Char('i') => {
                        state.active_menu = MenuItem::Intro;
                        if !state.connected {
                            state.input_mode = InputMode::User;
                        }
                    },
                    KeyCode::Char('r') => state.active_menu = MenuItem::Summary,
                    KeyCode::Char('a') => state.active_menu = MenuItem::CurrentChallenge,
                    KeyCode::Char('s') => state.active_menu = MenuItem::Split,
                    _ => {}
                }
                InputMode::User => match event.code {
                    KeyCode::Enter => {
                        match connect("127.0.0.1:7878".to_string(), &state.name) {
                            Some(val) => {
                                state.stream = Some(val);
                                state.connected = true;
                                state.input_mode = InputMode::Normal;
                                state.active_menu = MenuItem::Summary;
                            },
                            None => {
                                state.error = Some(UIError::ConnectError)
                            }
                        }
                    }
                    KeyCode::Char(c) => state.name.push(c),
                    KeyCode::Backspace => {
                        state.name.pop();
                    },
                    KeyCode::Esc => {
                        close_term(&mut state.term);
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