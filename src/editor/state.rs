use crate::terminal::Terminal;
use crate::buffer::GapBuffer;
use crate::events::EventLoop;
use crate::bridge::IntelligenceBridge;
use crate::highlight::Highlighter;
use crate::fuzzy::FuzzySearch;
use std::error::Error;
use std::time::Instant;

pub struct Editor {
    pub(crate) terminal: Terminal,
    pub(crate) buffer: GapBuffer,
    pub(crate) event_loop: EventLoop,
    pub(crate) bridge: IntelligenceBridge,
    pub(crate) highlighter: Highlighter,
    pub(crate) fuzzy: FuzzySearch,
    pub(crate) rowoff: usize,
    pub(crate) coloff: usize,
    pub(crate) show_fuzzy: bool,
    pub(crate) fuzzy_query: String,
    pub(crate) is_searching: bool,
    pub(crate) search_query: String,
    pub(crate) is_command_mode: bool,
    pub(crate) command_buffer: String,
    pub(crate) status_msg: String,
    pub(crate) status_time: Instant,
    pub(crate) show_help_overlay: bool,
}

impl Editor {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            terminal: Terminal::new()?,
            buffer: GapBuffer::new(1024),
            event_loop: EventLoop::new(50),
            bridge: IntelligenceBridge::new(),
            highlighter: Highlighter::new(),
            fuzzy: FuzzySearch::new(vec![
                "src/main.rs".to_string(), "src/buffer.rs".to_string(), 
                "src/editor/mod.rs".to_string(), "intelligence/senses.py".to_string()
            ]),
            rowoff: 0, coloff: 0,
            show_fuzzy: false, fuzzy_query: String::new(),
            is_searching: false, search_query: String::new(),
            is_command_mode: false, command_buffer: String::new(),
            status_msg: String::from("HELP: : for command | Ctrl-Q = quit | Ctrl-F = find"),
            status_time: Instant::now(),
            show_help_overlay: false,
        })
    }
}
