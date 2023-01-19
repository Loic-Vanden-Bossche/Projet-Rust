use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};
use crossterm::event;
use crossterm::event::{Event as CEvent, KeyEvent};

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