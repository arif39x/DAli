use crate::buffer::GapBuffer;
use crate::editor::state::Editor;
use crate::editor::types::{ViewMode, Window};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::error::Error;
use std::time::Instant;

impl Editor {
    pub fn process_keypress(&mut self, event: Event) -> Result<bool, Box<dyn Error>> {
        match event {
            Event::Key(key) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    let kb = &self.config.keybindings;
                    let code = key.code;
                    let mods = key.modifiers;

                    if kb.matches(&kb.quit, code, mods) {
                        return Ok(true);
                    }
                    if kb.matches(&kb.fuzzy, code, mods) {
                        self.show_fuzzy = !self.show_fuzzy;
                        self.fuzzy_query.clear();
                        return Ok(false);
                    }
                    if kb.matches(&kb.find, code, mods) {
                        self.is_searching = true;
                        self.search_query.clear();
                        return Ok(false);
                    }
                    if kb.matches(&kb.save, code, mods) {
                        self.status_msg = "File saved".to_string();
                        self.status_time = Instant::now();
                    }
                    if kb.matches(&kb.command, code, mods) {
                        self.is_command_mode = true;
                        self.command_buffer.clear();
                        return Ok(false);
                    }

                    match code {
                        KeyCode::Char('z') => {
                            self.focused_window().buffer.undo();
                            self.focused_window().dirty = true;
                        }
                        KeyCode::Char('y') => {
                            self.focused_window().buffer.redo();
                            self.focused_window().dirty = true;
                        }
                        KeyCode::Char('w') => {
                            self.focused_index = (self.focused_index + 1) % self.windows.len();
                            return Ok(false);
                        }
                        KeyCode::Char('e') => {
                            self.is_command_mode = true;
                            self.command_buffer.clear();
                            return Ok(false);
                        }
                        KeyCode::Char('c') => {
                            self.copy_selection();
                            return Ok(false);
                        }
                        KeyCode::Char('x') => {
                            self.cut_selection();
                            self.focused_window().dirty = true;
                            return Ok(false);
                        }
                        KeyCode::Char('v') => {
                            self.paste_from_clipboard();
                            self.focused_window().dirty = true;
                            return Ok(false);
                        }
                        _ => {}
                    }
                }
                if self.is_searching {
                    self.handle_search(key.code);
                    return Ok(false);
                }
                if self.show_help_overlay {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                            self.show_help_overlay = false;
                            self.help_scroll = 0;
                        }
                        KeyCode::Up => {
                            self.help_scroll = self.help_scroll.saturating_sub(1);
                        }
                        KeyCode::Down => {
                            self.help_scroll += 1;
                        }
                        _ => {}
                    }
                    return Ok(false);
                }
                if self.is_command_mode {
                    if self.handle_command(key.code) {
                        return Ok(true);
                    }
                    return Ok(false);
                }
                if self.show_fuzzy {
                    self.handle_fuzzy(key.code);
                    return Ok(false);
                }
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
                    KeyCode::Char(c) => {
                        self.focused_window().buffer.insert(c);
                        self.focused_window().dirty = true;
                    }
                    KeyCode::Backspace => {
                        self.focused_window().buffer.delete();
                        self.focused_window().dirty = true;
                    }
                    KeyCode::Enter => {
                        self.handle_enter();
                        self.focused_window().dirty = true;
                    }
                    KeyCode::Tab => {
                        if key.modifiers.contains(KeyModifiers::SHIFT) {
                            self.handle_outdent();
                        } else {
                            self.handle_snippet();
                        }
                        self.focused_window().dirty = true;
                    }
                    KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                        let is_shift = key.modifiers.contains(KeyModifiers::SHIFT);
                        let idx = self.focused_index;
                        if is_shift && self.windows[idx].selection_start.is_none() {
                            self.windows[idx].selection_start =
                                Some(self.windows[idx].buffer.cursor_pos());
                        } else if !is_shift {
                            self.windows[idx].selection_start = None;
                        }

                        match key.code {
                            KeyCode::Left => self.windows[idx].buffer.move_left(),
                            KeyCode::Right => self.windows[idx].buffer.move_right(),
                            KeyCode::Up => {
                                let (r, c) = self
                                    .get_cursor_coords(self.windows[idx].buffer.cursor_pos(), idx);
                                if r > 0 {
                                    let new_idx = self.find_index_by_coords(idx, r - 1, c);
                                    self.windows[idx].buffer.move_cursor(new_idx);
                                }
                            }
                            KeyCode::Down => {
                                let (r, c) = self
                                    .get_cursor_coords(self.windows[idx].buffer.cursor_pos(), idx);
                                let new_idx = self.find_index_by_coords(idx, r + 1, c);
                                self.windows[idx].buffer.move_cursor(new_idx);
                            }
                            _ => {}
                        }
                        self.windows[idx].dirty = true;
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
                if let Some(idx) = self
                    .focused_window_ref()
                    .buffer
                    .content()
                    .find(&self.search_query)
                {
                    let f_idx = self.focused_index;
                    self.windows[f_idx].buffer.move_cursor(idx);
                    self.windows[f_idx].dirty = true;
                }
            }
            KeyCode::Backspace => {
                self.search_query.pop();
            }
            KeyCode::Esc | KeyCode::Enter => {
                self.is_searching = false;
            }
            _ => {}
        }
    }

    pub(crate) fn handle_command(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Char(c) => {
                self.command_buffer.push(c);
                false
            }
            KeyCode::Backspace => {
                self.command_buffer.pop();
                false
            }
            KeyCode::Esc => {
                self.is_command_mode = false;
                false
            }
            KeyCode::Enter => {
                let cmd_raw = self.command_buffer.trim();
                let tokens: Vec<&str> = cmd_raw.split_whitespace().collect();
                if tokens.is_empty() {
                    return false;
                }

                match tokens[0] {
                    "q" => return true,
                    "s" => {
                        self.status_msg = "SUCCESS: File Saved".to_string();
                        self.status_time = Instant::now();
                    }
                    "h" | "help" => {
                        self.show_help_overlay = true;
                    }
                    "vsplit" => {
                        let focused = self.focused_window_ref();
                        let new_window = Window {
                            buffer: GapBuffer::new(1024),
                            viewport: focused.viewport,
                            rowoff: 0,
                            coloff: 0,
                            filename: focused.filename.clone(),
                            selection_start: None,
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
                        self.file_list = std::fs::read_dir(".")
                            .unwrap()
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
                    "build" => {
                        self.status_msg = "BUILDING: Running cargo build...".to_string();
                        self.status_time = Instant::now();
                    }
                    s if s.chars().all(|c| c.is_ascii_digit()) => {
                        if let Ok(num) = s.parse::<usize>() {
                            if num > 0 && num <= self.windows.len() {
                                self.focused_index = num - 1;
                                self.status_msg = format!("Switched to window {}", num);
                                self.status_time = Instant::now();
                                for w in &mut self.windows {
                                    w.dirty = true;
                                }
                            } else {
                                self.status_msg = format!("ERROR: Invalid window number {}", num);
                                self.status_time = Instant::now();
                            }
                        }
                    }
                    _ => {
                        self.status_msg = format!("ERROR: Unknown command '{}'", cmd_raw);
                        self.status_time = Instant::now();
                    }
                }
                self.is_command_mode = false;
                self.command_buffer.clear();
                false
            }
            _ => false,
        }
    }

    pub(crate) fn handle_fuzzy(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(c) => {
                self.fuzzy_query.push(c);
            }
            KeyCode::Backspace => {
                self.fuzzy_query.pop();
            }
            KeyCode::Esc => {
                self.show_fuzzy = false;
            }
            _ => {}
        }
    }

    pub(crate) fn handle_enter(&mut self) {
        let f_idx = self.focused_index;
        let content = self.windows[f_idx].buffer.content().to_string();
        let (r, _) = self.get_cursor_coords(self.windows[f_idx].buffer.cursor_pos(), f_idx);
        self.windows[f_idx].buffer.insert('\n');

        let indent = self.text_ops.calculate_indent(&content, r + 1);
        for _ in 0..indent {
            self.windows[f_idx].buffer.insert(' ');
        }
    }

    pub(crate) fn handle_snippet(&mut self) {
        let f_idx = self.focused_index;
        if let Some((start, end)) = self.get_selection_range() {
            self.indent_selection(start, end);
            return;
        }

        let cursor_pos = self.windows[f_idx].buffer.cursor_pos();
        let content = self.windows[f_idx].buffer.content();
        let mut start = cursor_pos;
        let bytes = content.as_bytes();
        while start > 0 && (bytes[start - 1] as char).is_alphanumeric() {
            start -= 1;
        }

        if start < cursor_pos {
            let trigger = &content[start..cursor_pos];
            if let Some(expanded) = self.text_ops.expand_snippet(trigger) {
                // Delete trigger
                for _ in 0..trigger.len() {
                    self.windows[f_idx].buffer.delete();
                }
                // Insert expanded (simple insert for now, no placeholder logic)
                for c in expanded.chars() {
                    self.windows[f_idx].buffer.insert(c);
                }
            } else {
                for _ in 0..4 {
                    self.windows[f_idx].buffer.insert(' ');
                }
            }
        } else {
            for _ in 0..4 {
                self.windows[f_idx].buffer.insert(' ');
            }
        }
    }

    pub(crate) fn handle_outdent(&mut self) {
        let f_idx = self.focused_index;
        if let Some((start, end)) = self.get_selection_range() {
            self.outdent_selection(start, end);
            return;
        }

        let cursor_pos = self.windows[f_idx].buffer.cursor_pos();
        let (r, _) = self.get_cursor_coords(cursor_pos, f_idx);
        let line_start = self.windows[f_idx].buffer.line_starts[r];
        let content = self.windows[f_idx].buffer.content();
        let line = content.lines().nth(r).unwrap_or("");

        let leading_spaces = line.chars().take_while(|c| c.is_whitespace()).count();
        let to_remove = leading_spaces.min(4);
        if to_remove > 0 {
            self.windows[f_idx].buffer.move_cursor(line_start);
            for _ in 0..to_remove {
                // need a delete_forward or something, or move cursor and then delete.
                // :delete deletes backwards.
                self.windows[f_idx].buffer.move_cursor(line_start + 1);
                self.windows[f_idx].buffer.delete();
            }
            self.windows[f_idx]
                .buffer
                .move_cursor(cursor_pos.saturating_sub(to_remove));
        }
    }

    fn get_selection_range(&self) -> Option<(usize, usize)> {
        let window = &self.windows[self.focused_index];
        window.selection_start.map(|start| {
            let end = window.buffer.cursor_pos();
            if start < end {
                (start, end)
            } else {
                (end, start)
            }
        })
    }

    fn copy_selection(&mut self) {
        if let Some((start, end)) = self.get_selection_range() {
            let content = self.windows[self.focused_index].buffer.content();
            if let Some(selected_text) = content.get(start..end) {
                let _ = self.clipboard.set_text(selected_text.to_string());
                self.status_msg = "Copied to clipboard".to_string();
                self.status_time = Instant::now();
            }
        }
    }

    fn cut_selection(&mut self) {
        if let Some((start, end)) = self.get_selection_range() {
            self.copy_selection();
            self.delete_range(start, end);
            self.windows[self.focused_index].selection_start = None;
        }
    }

    fn paste_from_clipboard(&mut self) {
        if let Ok(text) = self.clipboard.get_text() {
            if let Some((start, end)) = self.get_selection_range() {
                self.delete_range(start, end);
                self.windows[self.focused_index].selection_start = None;
            }
            for c in text.chars() {
                self.windows[self.focused_index].buffer.insert(c);
            }
        }
    }

    fn delete_range(&mut self, start: usize, end: usize) {
        let window = &mut self.windows[self.focused_index];
        window.buffer.move_cursor(end);
        for _ in 0..(end - start) {
            window.buffer.delete();
        }
    }

    fn indent_selection(&mut self, start: usize, end: usize) {
        let f_idx = self.focused_index;
        let (start_row, _) = self.get_cursor_coords(start, f_idx);
        let (end_row, _) = self.get_cursor_coords(end, f_idx);

        for r in (start_row..=end_row).rev() {
            let line_start = self.windows[f_idx].buffer.line_starts[r];
            self.windows[f_idx].buffer.move_cursor(line_start);
            for _ in 0..4 {
                self.windows[f_idx].buffer.insert(' ');
            }
        }
        self.windows[f_idx].selection_start = None;
    }

    fn outdent_selection(&mut self, start: usize, end: usize) {
        let f_idx = self.focused_index;
        let (start_row, _) = self.get_cursor_coords(start, f_idx);
        let (end_row, _) = self.get_cursor_coords(end, f_idx);

        for r in (start_row..=end_row).rev() {
            let line_start = self.windows[f_idx].buffer.line_starts[r];
            let content = self.windows[f_idx].buffer.content();
            let line = content.lines().nth(r).unwrap_or("");
            let leading_spaces = line.chars().take_while(|c| c.is_whitespace()).count();
            let to_remove = leading_spaces.min(4);

            self.windows[f_idx]
                .buffer
                .move_cursor(line_start + to_remove);
            for _ in 0..to_remove {
                self.windows[f_idx].buffer.delete();
            }
        }
        self.windows[f_idx].selection_start = None;
    }
}
