pub mod state;
pub mod input;
pub mod render;
pub mod help;

pub use state::Editor;
use std::error::Error;
use crate::highlight::Highlighter;

impl Editor {
    pub fn draw_view(&mut self, screen_rows: u16, screen_cols: u16, cur_row: usize, cur_col: usize) -> Result<(), Box<dyn Error>> {
        if self.show_help_overlay { return self.draw_help_overlay(screen_rows, screen_cols); }
        self.terminal.move_cursor(0, 0);
        let tokens = self.highlighter.highlight(&self.buffer.content());
        let mut r_row = 0; let mut r_col = 0;
        self.draw_margin();
        
        for (text, token_type) in tokens {
            let color = Highlighter::get_color(token_type);
            self.terminal.set_color_24bit(color.0, color.1, color.2);
            for c in text.chars() {
                if c == '\n' {
                    if r_row >= self.rowoff && r_row < self.rowoff + screen_rows as usize { self.terminal.clear_from_cursor_to_end(); }
                    r_row += 1; r_col = 0;
                    if r_row >= self.rowoff && r_row < self.rowoff + screen_rows as usize {
                        self.terminal.move_cursor(0, (r_row - self.rowoff) as u16);
                        self.draw_margin();
                    }
                } else {
                    if r_row >= self.rowoff && r_row < self.rowoff + screen_rows as usize {
                        if r_col >= self.coloff && r_col < self.coloff + (screen_cols - 2) as usize {
                            self.terminal.write_content(&c.to_string());
                        }
                    }
                    r_col += 1;
                }
            }
        }
        self.draw_empty_tildes(r_row, screen_rows);
        self.draw_status_bar(screen_rows, screen_cols, cur_row, cur_col);
        self.draw_fuzzy_panel(screen_rows, screen_cols);
        self.draw_command_bar(screen_rows, screen_cols, cur_row, cur_col);
        self.terminal.flush()?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let (cols, rows) = self.terminal.size().unwrap_or((80, 24));
            let screen_rows = rows.saturating_sub(3);
            let (cur_row, cur_col) = self.get_cursor_coords(&self.buffer.content(), self.buffer.cursor_pos());
            self.editor_scroll(cur_row, cur_col, screen_rows, cols);
            if let Some(event) = self.event_loop.poll_event()? {
                if self.process_keypress(event)? { break; }
            }
            if self.event_loop.should_tick() {
                self.draw_view(screen_rows, cols, cur_row, cur_col)?;
            }
        }
        Ok(())
    }

    fn get_cursor_coords(&self, content: &str, cursor_idx: usize) -> (usize, usize) {
        let mut r = 0; let mut c = 0;
        for (idx, ch) in content.chars().enumerate() {
            if idx == cursor_idx { break; }
            if ch == '\n' { r += 1; c = 0; } else { c += 1; }
        }
        (r, c)
    }

    fn editor_scroll(&mut self, cur_row: usize, cur_col: usize, screen_rows: u16, screen_cols: u16) {
        if cur_row < self.rowoff { self.rowoff = cur_row; }
        if cur_row >= self.rowoff + screen_rows as usize { self.rowoff = cur_row - screen_rows as usize + 1; }
        if cur_col < self.coloff { self.coloff = cur_col; }
        if cur_col >= self.coloff + (screen_cols.saturating_sub(2)) as usize { self.coloff = cur_col - (screen_cols.saturating_sub(2)) as usize + 1; }
    }

    pub(crate) fn find_index_by_coords(&self, target_row: usize, target_col: usize) -> usize {
        let content = self.buffer.content();
        let (mut r, mut c) = (0, 0);
        for (idx, ch) in content.chars().enumerate() {
            if r == target_row && c == target_col { return idx; }
            if ch == '\n' { if r == target_row { return idx; } r += 1; c = 0; } else { c += 1; }
        }
        if r == target_row { return content.len(); }
        0
    }
}
