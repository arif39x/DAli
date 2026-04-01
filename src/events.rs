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

    pub fn poll_event(&mut self) -> io::Result<Option<Event>> {
        let timeout = self.tick_rate
            .checked_sub(self.last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));

        if event::poll(timeout)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }

    pub fn should_tick(&mut self) -> bool {
        if self.last_tick.elapsed() >= self.tick_rate {
            self.last_tick = Instant::now();
            true
        } else {
            false
        }
    }
}
