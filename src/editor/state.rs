use crate::terminal::Terminal;
use crate::buffer::GapBuffer;
use crate::events::EventLoop;
use crate::bridge::IntelligenceBridge;
use crate::highlight::Highlighter;
use crate::fuzzy::FuzzySearch;
use std::error::Error;
use std::time::Instant;

use super::types::{ViewMode, Rect, Window};

pub struct Editor {
    pub(crate) terminal: Terminal,
    pub(crate) windows: Vec<Window>,
    pub(crate) focused_index: usize,
    pub(crate) view_mode: ViewMode,
    pub(crate) event_loop: EventLoop,
    pub(crate) bridge: IntelligenceBridge,
    pub(crate) highlighter: Highlighter,
    pub(crate) fuzzy: FuzzySearch,
    pub(crate) clipboard: arboard::Clipboard,
    pub(crate) show_fuzzy: bool,
    pub(crate) fuzzy_query: String,
    pub(crate) is_searching: bool,
    pub(crate) search_query: String,
    pub(crate) is_command_mode: bool,
    pub(crate) command_buffer: String,
    pub(crate) file_list: Vec<String>,
    pub(crate) status_msg: String,
    pub(crate) status_time: Instant,
    pub(crate) show_help_overlay: bool,
    pub(crate) last_git_check: Instant,
    pub(crate) git_branch: String,
    pub(crate) git_modified: usize,
    pub(crate) last_size: (u16, u16),
    pub(crate) help_scroll: usize,
}


impl Editor {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (cols, rows) = Terminal::new()?.size().unwrap_or((80, 24));
        let screen_rows = rows.saturating_sub(3);
        
        let initial_window = Window {
            buffer: GapBuffer::new(1024),
            terminal_state: None,
            viewport: Rect { x: 0, y: 0, width: cols, height: screen_rows },
            rowoff: 0, coloff: 0,
            filename: "untitled.rs".to_string(),
            selection_start: None,
            dirty: true,
        };

        Ok(Self {
            terminal: Terminal::new()?,
            windows: vec![initial_window],
            focused_index: 0,
            view_mode: ViewMode::Editor,
            event_loop: EventLoop::new(50),
            bridge: IntelligenceBridge::new(),
            highlighter: Highlighter::new(),
            fuzzy: FuzzySearch::new(vec![
                "src/main.rs".to_string(), "src/buffer.rs".to_string(), 
                "src/editor/mod.rs".to_string(), "intelligence/senses.py".to_string()
            ]),
            clipboard: arboard::Clipboard::new().expect("Failed to initialize clipboard"),
            show_fuzzy: false, fuzzy_query: String::new(),
            file_list: Vec::new(),
            is_searching: false, search_query: String::new(),
            is_command_mode: false, command_buffer: String::new(),
            status_msg: String::from("HELP: : for command | Ctrl-Q = quit | Ctrl-F = find"),
            status_time: Instant::now(),
            show_help_overlay: false,
            last_git_check: Instant::now(),
            git_branch: "No Git".to_string(),
            git_modified: 0,
            last_size: (cols, rows),
            help_scroll: 0,
        })
    }
}
