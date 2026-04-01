use crate::editor::Editor;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::error::Error;
use std::time::Instant;

impl Editor {
    pub fn process_keypress(&mut self, event: Event) -> Result<bool, Box<dyn Error>> {
        match event {
            Event::Key(key) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match key.code {
                        KeyCode::Char('q') => return Ok(true),
                        KeyCode::Char('p') => { self.show_fuzzy = !self.show_fuzzy; self.fuzzy_query.clear(); return Ok(false); }
                        KeyCode::Char('f') => { self.is_searching = true; self.search_query.clear(); return Ok(false); }
                        KeyCode::Char('s') => { self.status_msg = "File saved (v0.2.3)".to_string(); self.status_time = Instant::now(); }
                        _ => {}
                    }
                }
                if self.is_searching { self.handle_search(key.code); return Ok(false); }
                if self.show_help_overlay {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => { self.show_help_overlay = false; }
                        _ => {}
                    }
                    return Ok(false);
                }
                if self.is_command_mode { if self.handle_command(key.code) { return Ok(true); } return Ok(false); }
                if self.show_fuzzy { self.handle_fuzzy(key.code); return Ok(false); }

                match key.code {
                    KeyCode::Char(':') if !self.is_searching && !self.show_fuzzy && !self.show_help_overlay => { 
                        self.is_command_mode = true; 
                        self.command_buffer.clear(); 
                    }
                    KeyCode::Char(c) => self.buffer.insert(c),
                    KeyCode::Backspace => self.buffer.delete(),
                    KeyCode::Enter => self.handle_enter(),
                    KeyCode::Left => { let pos = self.buffer.cursor_pos(); if pos > 0 { self.buffer.move_cursor(pos - 1); } }
                    KeyCode::Right => { let pos = self.buffer.cursor_pos(); self.buffer.move_cursor(pos + 1); }
                    KeyCode::Up => {
                        let content = self.buffer.content();
                        let (r, c) = self.get_cursor_coords(&content, self.buffer.cursor_pos());
                        if r > 0 { let new_idx = self.find_index_by_coords(r - 1, c); self.buffer.move_cursor(new_idx); }
                    }
                    KeyCode::Down => {
                        let content = self.buffer.content();
                        let (r, c) = self.get_cursor_coords(&content, self.buffer.cursor_pos());
                        let new_idx = self.find_index_by_coords(r + 1, c); 
                        self.buffer.move_cursor(new_idx);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(false)
    }

    pub(crate) fn handle_search(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(c) => {
                self.search_query.push(c);
                if let Some(idx) = self.buffer.content().find(&self.search_query) { self.buffer.move_cursor(idx); }
            }
            KeyCode::Backspace => { self.search_query.pop(); }
            KeyCode::Esc | KeyCode::Enter => { self.is_searching = false; }
            _ => {}
        }
    }

    pub(crate) fn handle_command(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Char(c) => { self.command_buffer.push(c); false }
            KeyCode::Backspace => { self.command_buffer.pop(); false }
            KeyCode::Esc => { self.is_command_mode = false; false }
            KeyCode::Enter => {
                let cmd = self.command_buffer.trim();
                match cmd {
                    "q" => return true,
                    "s" => { self.status_msg = "SUCCESS: File Saved".to_string(); self.status_time = Instant::now(); }
                    "h" | "help" => { self.show_help_overlay = true; }
                    "build" => { self.status_msg = "BUILDING: Running cargo build...".to_string(); self.status_time = Instant::now(); }
                    _ => { self.status_msg = format!("ERROR: Unknown command '{}'", cmd); self.status_time = Instant::now(); }
                }
                self.is_command_mode = false; self.command_buffer.clear(); false
            }
            _ => false,
        }
    }

    pub(crate) fn handle_fuzzy(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(c) => { self.fuzzy_query.push(c); }
            KeyCode::Backspace => { self.fuzzy_query.pop(); }
            KeyCode::Esc => { self.show_fuzzy = false; }
            _ => {}
        }
    }

    pub(crate) fn handle_enter(&mut self) {
        self.buffer.insert('\n');
        if let Ok(indent) = self.bridge.calculate_indent(&self.buffer.content()) {
            for _ in 0..indent { self.buffer.insert(' '); }
        }
    }
}
