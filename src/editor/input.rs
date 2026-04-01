use crate::buffer::GapBuffer;
use crate::editor::state::Editor;
use crate::editor::types::{ViewMode, Window, TerminalState};
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
                        KeyCode::Char('s') => { self.status_msg = "File saved".to_string(); self.status_time = Instant::now(); }
                        KeyCode::Char('z') => { self.focused_window().buffer.undo(); self.focused_window().dirty = true; }
                        KeyCode::Char('y') => { self.focused_window().buffer.redo(); self.focused_window().dirty = true; }
                        KeyCode::Char('w') => { self.focused_index = (self.focused_index + 1) % self.windows.len(); return Ok(false); }
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
                if self.view_mode != ViewMode::Editor {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => { 
                            self.view_mode = ViewMode::Editor; 
                            let idx = self.focused_index;
                            self.windows[idx].dirty = true; 
                        }
                        _ => {}
                    }
                    return Ok(false);
                }

                match key.code {
                    KeyCode::Char(':') if !self.is_searching && !self.show_fuzzy && !self.show_help_overlay => { 
                        self.is_command_mode = true; 
                        self.command_buffer.clear(); 
                    }
                    KeyCode::Char(c) => { self.focused_window().buffer.insert(c); self.focused_window().dirty = true; }
                    KeyCode::Backspace => { self.focused_window().buffer.delete(); self.focused_window().dirty = true; }
                    KeyCode::Enter => { self.handle_enter(); self.focused_window().dirty = true; }
                    KeyCode::Tab => { self.handle_snippet(); self.focused_window().dirty = true; }
                    KeyCode::Left => { self.focused_window().buffer.move_left(); self.focused_window().dirty = true; }
                    KeyCode::Right => { self.focused_window().buffer.move_right(); self.focused_window().dirty = true; }
                    KeyCode::Up => {
                        let idx = self.focused_index;
                        let (r, c) = self.get_cursor_coords(self.windows[idx].buffer.cursor_pos(), idx);
                        if r > 0 { let new_idx = self.find_index_by_coords(idx, r - 1, c); self.windows[idx].buffer.move_cursor(new_idx); self.windows[idx].dirty = true; }
                    }
                    KeyCode::Down => {
                        let idx = self.focused_index;
                        let (r, c) = self.get_cursor_coords(self.windows[idx].buffer.cursor_pos(), idx);
                        let new_idx = self.find_index_by_coords(idx, r + 1, c); 
                        self.windows[idx].buffer.move_cursor(new_idx); self.windows[idx].dirty = true;
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
                if let Some(idx) = self.focused_window_ref().buffer.content().find(&self.search_query) {
                    let f_idx = self.focused_index;
                    self.windows[f_idx].buffer.move_cursor(idx); 
                    self.windows[f_idx].dirty = true;
                }
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
                    "vsplit" => {
                        let args: Vec<&str> = self.command_buffer.split_whitespace().skip(1).collect();
                        let is_term = args.get(0) == Some(&"term");
                        let focused = self.focused_window_ref();
                        let new_window = Window {
                            buffer: GapBuffer::new(1024),
                            terminal_state: if is_term { Some(TerminalState { buffer: vec!["DAli Terminal v0.2".to_string(), "Type commands here...".to_string()] }) } else { None },
                            viewport: focused.viewport,
                            rowoff: 0, coloff: 0,
                            filename: if is_term { "terminal".to_string() } else { focused.filename.clone() },
                            dirty: true,
                        };
                        self.windows.push(new_window);
                        self.focused_index = self.windows.len() - 1;
                        self.is_command_mode = false;
                    }
                    "pwd" => {
                        let path = std::env::current_dir().unwrap_or_default();
                        self.status_msg = format!("PWD: {}", path.display());
                        self.status_time = Instant::now();
                    }
                    "ls" => {
                        self.view_mode = ViewMode::FileList;
                        self.file_list = std::fs::read_dir(".").unwrap()
                            .filter_map(|e| e.ok())
                            .map(|e| e.file_name().to_string_lossy().to_string())
                            .collect();
                        let idx = self.focused_index;
                        self.windows[idx].dirty = true;
                    }
                    "tree" => {
                        self.view_mode = ViewMode::FileList;
                        self.file_list = self.get_tree_view(".", 0);
                        let idx = self.focused_index;
                        self.windows[idx].dirty = true;
                    }
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
        let f_idx = self.focused_index;
        let (r, _) = self.get_cursor_coords(self.windows[f_idx].buffer.cursor_pos(), f_idx);
        self.windows[f_idx].buffer.insert('\n');
        self.bridge.request_indent(&self.windows[f_idx].buffer, r + 1);
    }

    pub(crate) fn handle_snippet(&mut self) {
        let f_idx = self.focused_index;
        let cursor_pos = self.windows[f_idx].buffer.cursor_pos();
        let content = self.windows[f_idx].buffer.content();
        let mut start = cursor_pos;
        let bytes = content.as_bytes();
        while start > 0 && (bytes[start-1] as char).is_alphanumeric() {
            start -= 1;
        }
        
        if start < cursor_pos {
            let trigger = &content[start..cursor_pos];
            self.bridge.request_snippet(trigger);
            // Snippets are now async andle the response in handle_bridge_msg
        } else {
            for _ in 0..4 { self.windows[f_idx].buffer.insert(' '); }
        }
    }
}
