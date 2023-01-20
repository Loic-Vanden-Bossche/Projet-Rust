use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};
use crossterm::event;
use crossterm::event::{Event as CEvent, KeyCode, KeyEvent};
use log::{debug, error};
use crate::function::connect::connect;
use crate::tui::error::UIError;
use crate::tui::input::InputMode;
use crate::tui::menu::MenuItem;
use crate::tui::term::close_term;
use crate::tui::ui::{State, Term};
use crate::types::challenge::Challenge;
use crate::types::end::EndOfGame;
use crate::types::player::PublicLeaderBoard;
use crate::types::round::RoundSummary;

pub enum GameEvent{
    PublicLeaderBoard(PublicLeaderBoard),
    ChallengeInput(Challenge),
    EndOfRound(RoundSummary),
    EndOfGame(EndOfGame),
    Error(UIError)
}

pub enum Event<I> {
    Game(GameEvent),
    Input(I),
    Tick,
}

pub fn event_loop(tx: Sender<Event<KeyEvent>>){
    let tick_rate = Duration::from_millis(100);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = if let Some(v) = tick_rate.checked_sub(last_tick.elapsed()){
                v
            }else {
                Duration::from_secs(0)
            };
            match event::poll(timeout) {
                Ok(res) => {
                    if res {
                        match event::read(){
                            Ok(CEvent::Key(key)) => {
                                send(&tx, Event::Input(key));
                            }
                            Err(err) => {
                                error!("Error reading event: {err}");
                                return;
                            }
                            _ => {}
                        };
                    }
                }
                Err(err) => {
                    error!("Can't poll event: {err}");
                    send(&tx, Event::Game(GameEvent::Error(UIError::FatalError)));
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

pub fn receive_event(rx: &Receiver<Event<KeyEvent>>, sS: &Sender<(TcpStream, String)>, state: &mut State, term: &mut Term, url: &String) -> bool {
    let event = match rx.recv() {
        Ok(e) => {
            debug!("Successfully received event");
            e
        }
        Err(err) => {
            state.error = Some(UIError::FatalError);
            error!("On event receiving : {err}");
            return true;
        }
    };
    match event {
        Event::Input(event) => {
            match state.input_mode {
                InputMode::Normal => match event.code {
                    KeyCode::Char('q') => {
                        close_term(term);
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
                        match connect(url.clone(), &state.name) {
                            Some(val) => {
                                match sS.send((val, state.name.clone())) {
                                    Ok(_) => {
                                        debug!("Stream and name send to game thread");
                                    }
                                    Err(_) => {
                                        error!("Unable to share stream and name with thread [game]")
                                    }
                                }
                                state.connected = true;
                                state.error = None;
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
                        close_term(term);
                        return false;
                    }
                    _ => {}
                }
            }
        },
        Event::Tick => {},
        Event::Game(ev) => match ev {
            GameEvent::EndOfGame(eog) => {
                state.eog = Some(eog);
                state.error = None;
            },
            GameEvent::PublicLeaderBoard(plb) => {
                state.summary = Some(plb);
                state.error = None;
            }
            GameEvent::Error(e) => {
                state.error = Some(e);
            }
            GameEvent::ChallengeInput(val) => {
                state.current = Some(val);
                state.error = None;
            }
            GameEvent::EndOfRound(_) => {
                state.error = None;
                debug!("End of round");
            }
        }
    };
    return true;
}

pub fn send<T>(sender: &Sender<T>, data: T) -> bool{
    match sender.send(data) {
        Ok(_) => {
            debug!("Data sent");
            true
        }
        Err(err) => {
            error!("Error sending data : {err}");
            false
        }
    }
}