use crate::editor::Editor;

impl Editor {
    pub(crate) fn draw_margin(&mut self) {
        self.terminal.set_color_24bit(100, 100, 100);
        self.terminal.write_content("~ ");
        self.terminal.reset_color();
    }

    pub(crate) fn draw_empty_tildes(&mut self, r_row: usize, screen_rows: u16) {
        let visible_rows = r_row.saturating_sub(self.rowoff);
        for r in (visible_rows + 1)..screen_rows as usize {
            self.terminal.move_cursor(0, r as u16);
            self.terminal.set_color_24bit(100, 100, 100);
            self.terminal.write_content("~");
            self.terminal.reset_color();
            self.terminal.clear_line();
        }
    }

    pub(crate) fn draw_status_bar(&mut self, screen_rows: u16, screen_cols: u16, cur_row: usize, cur_col: usize) {
        let rows = screen_rows + 3;
        self.terminal.move_cursor(0, rows - 2);
        let mut status = format!(" [DAli] Row: {} Col: {} ", cur_row + 1, cur_col + 1);
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
            status = format!(" {} | {}", status, self.bridge.get_status_message());
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
            let (v_col, v_row) = ((cur_col as i32 - self.coloff as i32 + 2) as u16, (cur_row as i32 - self.rowoff as i32) as u16);
            self.terminal.move_cursor(v_col, v_row);
        }
        self.terminal.reset_color(); self.terminal.show_cursor();
    }
}
