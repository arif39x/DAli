use std::io;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event};

pub struct EventLoop {
    last_tick: Instant,
    tick_rate: Duration,
}

impl EventLoop {
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(tick_rate_ms),
        }
    }

    pub fn poll_event(&mut self, timeout: Duration) -> io::Result<Option<Event>> {
        if event::poll(timeout)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }

    // should_tick is deprecated, we use event-driven updates now
}
