use super::state::Editor;
use super::types::{Window, TerminalState};
use crate::highlight::Highlighter;
use std::error::Error;

impl Editor {
    pub(crate) fn draw_margin(&mut self, window: &Window) {
        self.terminal.move_cursor(window.viewport.x, window.viewport.y);
        self.terminal.set_color_24bit(100, 100, 100);
        self.terminal.write_content("~ ");
        self.terminal.reset_color();
    }

    pub(crate) fn draw_empty_tildes(&mut self, window: &Window, last_row: usize) {
        let viewport = window.viewport;
        for r in (last_row + 1)..viewport.height as usize {
            self.terminal.move_cursor(viewport.x, viewport.y + r as u16);
            self.terminal.set_color_24bit(100, 100, 100);
            self.terminal.write_content("~");
            self.terminal.reset_color();
            // Clear only within the viewport? Hard with standard clear_line.
            // I'll just write spaces for now or skip it if it's too complex.
        }
    }

    pub(crate) fn draw_status_bar(&mut self, screen_rows: u16, screen_cols: u16) {
        let focused_idx = self.focused_index;
        let focused = &self.windows[focused_idx];
        let (cur_row, cur_col) = self.get_cursor_coords(focused.buffer.cursor_pos(), focused_idx);

        let rows = screen_rows + 3;
        self.terminal.move_cursor(0, rows - 2);
        let mut status = format!(" [DAli] {} | Row: {} Col: {} ", focused.filename, cur_row + 1, cur_col + 1);
        if self.is_searching {
            self.terminal.set_color_24bit(255, 255, 100);
            status = format!(" SEARCH: {} (Esc to exit)", self.search_query);
        } else if self.status_time.elapsed().as_secs() < 5 {
            if self.status_msg.starts_with("HELP") { self.terminal.set_color_24bit(255, 255, 100); }
            else if self.status_msg.starts_with("ERROR") { self.terminal.set_color_24bit(255, 100, 100); }
            else if self.status_msg.starts_with("SUCCESS") { self.terminal.set_color_24bit(100, 255, 100); }
            else { self.terminal.set_color_24bit(0, 150, 150); }
            status = self.status_msg.clone();
        } else {
            self.terminal.set_color_24bit(0, 150, 150);
            status = format!(" {} | Git: {} (+{})", status, self.git_branch, self.git_modified);
        }
        let truncated = if status.len() > screen_cols as usize { &status[..screen_cols as usize] } else { &status };
        self.terminal.write_content(truncated);
        self.terminal.clear_from_cursor_to_end();
        self.terminal.reset_color();
    }

    pub(crate) fn draw_fuzzy_panel(&mut self, screen_rows: u16, _screen_cols: u16) {
        if self.show_fuzzy && screen_rows > 5 {
            let rows = screen_rows + 3; let fuzzy_row = rows.saturating_sub(8);
            self.terminal.move_cursor(0, fuzzy_row);
            self.terminal.set_color_24bit(255, 255, 100);
            self.terminal.write_content(&format!(" Fuzzy Search: {}", self.fuzzy_query));
            self.terminal.clear_from_cursor_to_end();
            let matches = self.fuzzy.search(&self.fuzzy_query, 5);
            for (i, (file, dist)) in matches.iter().enumerate() {
                if fuzzy_row + 1 + (i as u16) < rows - 3 {
                    self.terminal.move_cursor(0, fuzzy_row + 1 + (i as u16));
                    self.terminal.write_content(&format!("  {} (dist: {})", file, dist));
                    self.terminal.clear_from_cursor_to_end();
                }
            }
            self.terminal.reset_color();
        }
    }

    pub(crate) fn draw_command_bar(&mut self, screen_rows: u16, _screen_cols: u16, cur_row: usize, cur_col: usize) {
        let r = screen_rows + 3;
        self.terminal.move_cursor(0, r - 2);
        self.terminal.set_color_24bit(60, 60, 60);
        self.terminal.write_content(&"-".repeat(_screen_cols as usize));
        self.terminal.move_cursor(0, r - 1);
        self.terminal.set_color_24bit(30, 30, 30);
        self.terminal.write_content(&" ".repeat(_screen_cols as usize));
        self.terminal.move_cursor(0, r - 1);
        if self.is_command_mode {
            self.terminal.set_color_24bit(0, 255, 0); self.terminal.write_content(" $ ");
            self.terminal.set_color_24bit(255, 255, 255); self.terminal.write_content(&self.command_buffer);
            self.terminal.move_cursor((self.command_buffer.len() + 3) as u16, r - 1);
        } else {
            self.terminal.set_color_24bit(120, 120, 120);
            self.terminal.write_content(" $ (Press : for command)");
            let focused = &self.windows[self.focused_index];
            let (v_col, v_row) = ((cur_col as i32 - focused.coloff as i32 + 2) as u16 + focused.viewport.x, (cur_row as i32 - focused.rowoff as i32) as u16 + focused.viewport.y);
            self.terminal.move_cursor(v_col, v_row);
        }
        self.terminal.reset_color(); self.terminal.show_cursor();
    }

    pub(crate) fn draw_file_list(&mut self, screen_rows: u16, screen_cols: u16) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.move_cursor(0, 0);
        self.terminal.set_color_24bit(100, 255, 100);
        self.terminal.write_content(" --- FILE EXPLORER (Esc to exit) --- ");
        self.terminal.reset_color();
        self.terminal.clear_line();

        for (i, file) in self.file_list.iter().enumerate() {
            if (i + 1) >= screen_rows as usize { break; }
            self.terminal.move_cursor(0, (i + 1) as u16);
            if file.ends_with('/') {
                self.terminal.set_color_24bit(100, 150, 255); // Directory Blue
            } else {
                self.terminal.set_color_24bit(200, 200, 200); // File Gray
            }
            self.terminal.write_content(file);
            self.terminal.reset_color();
            self.terminal.clear_line();
        }
        
        for i in (self.file_list.len() + 1)..screen_rows as usize {
            if i >= screen_rows as usize { break; }
            self.terminal.move_cursor(0, i as u16);
            self.terminal.clear_line();
        }

        self.draw_status_bar(screen_rows, screen_cols);
        self.draw_command_bar(screen_rows, screen_cols, 0, 0);
        self.terminal.flush()?;
        Ok(())
    }

    pub(crate) fn draw_window(&mut self, index: usize) -> Result<(), Box<dyn Error>> {
        let (viewport, rowoff, coloff, _filename, extension, content) = {
            let window = &self.windows[index];
            if window.terminal_state.is_some() {
                let term_buffer = window.terminal_state.as_ref().unwrap().buffer.clone();
                return self.draw_terminal_window(index, term_buffer);
            }
            (
                window.viewport,
                window.rowoff,
                window.coloff,
                window.filename.clone(),
                window.filename.split('.').last().unwrap_or("rs").to_string(),
                window.buffer.content()
            )
        };

        self.draw_margin_at(viewport.x, viewport.y);
        let tokens = self.highlighter.highlight(&content, &extension);
        
        let mut r_row = 0;
        let mut r_col = 0;
        
        for (text, token_type) in tokens {
            let color = Highlighter::get_color(token_type);
            self.terminal.set_color_24bit(color.0, color.1, color.2);
            for c in text.chars() {
                if c == '\n' {
                    r_row += 1; r_col = 0;
                    if r_row >= rowoff && r_row < rowoff + viewport.height as usize {
                        self.draw_margin_at(viewport.x, viewport.y + (r_row - rowoff) as u16);
                    }
                } else {
                    if r_row >= rowoff && r_row < rowoff + viewport.height as usize {
                        if r_col >= coloff && r_col < coloff + (viewport.width.saturating_sub(2)) as usize {
                            let vx = viewport.x + (r_col - coloff) as u16 + 2;
                            let vy = viewport.y + (r_row - rowoff) as u16;
                            self.terminal.move_cursor(vx, vy);
                            self.terminal.write_content(&c.to_string());
                        }
                    }
                    r_col += 1;
                }
            }
        }
        
        let visible_row_count = (r_row + 1).saturating_sub(rowoff);
        for i in visible_row_count..viewport.height as usize {
             self.draw_margin_at(viewport.x, viewport.y + i as u16);
        }

        self.windows[index].dirty = false;
        Ok(())
    }

    pub(crate) fn draw_margin_at(&mut self, x: u16, y: u16) {
        self.terminal.move_cursor(x, y);
        self.terminal.set_color_24bit(100, 100, 100);
        self.terminal.write_content("~ ");
        self.terminal.reset_color();
    }
}
