pub mod state;
pub mod types;
pub mod window;
pub mod navigation;
pub mod terminal_window;
pub mod input;
pub mod render;
pub mod help;

pub use state::Editor;
pub use types::{ViewMode, Window};
use std::error::Error;

impl Editor {
    pub fn draw_view(&mut self, screen_rows: u16, screen_cols: u16) -> Result<(), Box<dyn Error>> {
        if self.show_help_overlay { return self.draw_help_overlay(screen_rows, screen_cols); }
        if self.view_mode == ViewMode::FileList {
            return self.draw_file_list(screen_rows, screen_cols);
        }

        // Only draw if any window is dirty or we are in a special mode
        let any_dirty = self.windows.iter().any(|w| w.dirty);
        if !any_dirty && !self.show_fuzzy && !self.is_command_mode && !self.is_searching {
            return Ok(());
        }

        for i in 0..self.windows.len() {
            self.draw_window(i)?;
        }

        let (cur_row, cur_col) = {
            let focused = &self.windows[self.focused_index];
            self.get_cursor_coords(focused.buffer.cursor_pos(), self.focused_index)
        };
        
        self.draw_status_bar(screen_rows, screen_cols);
        self.draw_fuzzy_panel(screen_rows, screen_cols);
        self.draw_command_bar(screen_rows, screen_cols, cur_row, cur_col);
        self.terminal.flush()?;
        Ok(())
    }


    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let (cols, rows) = self.terminal.size().unwrap_or((80, 24));
            let screen_rows = rows.saturating_sub(3);
            
            // Recalculate viewports if screen size changed
            self.recalculate_viewports(cols, screen_rows);
            
            let focused_idx = self.focused_index;
            let cursor_pos = self.windows[focused_idx].buffer.cursor_pos();
            let (cur_row, cur_col) = self.get_cursor_coords(cursor_pos, focused_idx);
            self.editor_scroll(focused_idx, cur_row, cur_col);
            
            self.draw_view(screen_rows, cols)?;
            
            // Check for background responses
            while let Some(res) = self.bridge.try_recv() {
                self.handle_bridge_msg(res);
            }
            
            // Periodic background tasks
            if self.last_git_check.elapsed().as_secs() > 10 {
                self.bridge.request_git_info();
                self.last_git_check = std::time::Instant::now();
            }
            
            // Block on events
            if let Some(event) = self.event_loop.poll_event(std::time::Duration::from_millis(50))? {
                if self.process_keypress(event)? { break; }
            }
        }
        Ok(())
    }

    fn handle_bridge_msg(&mut self, res: crate::bridge::BridgeResponse) {
        match res {
            crate::bridge::BridgeResponse::Indent(_indent) => {
                // To do :apply indent to focused window
            }
            crate::bridge::BridgeResponse::GitInfo(branch, modified) => {
                self.git_branch = branch;
                self.git_modified = modified;
                for w in &mut self.windows { w.dirty = true; }
            }
            crate::bridge::BridgeResponse::Snippet(_expanded) => {
                // To do implemented: handle snippet expansion
            }
        }
    }

    pub fn focused_window(&mut self) -> &mut Window {
        &mut self.windows[self.focused_index]
    }

    pub fn focused_window_ref(&self) -> &Window {
        &self.windows[self.focused_index]
    }

    fn get_cursor_coords(&self, cursor_idx: usize, window_idx: usize) -> (usize, usize) {
        let window = &self.windows[window_idx];
        let line_starts = &window.buffer.line_starts;
        let row = match line_starts.binary_search(&cursor_idx) {
            Ok(r) => r,
            Err(r) => r - 1,
        };
        
        let line_start = line_starts[row];
        let prefix = &window.buffer.content()[line_start..cursor_idx];
        let col = prefix.chars().count();
        (row, col)
    }

    fn editor_scroll(&mut self, window_idx: usize, cur_row: usize, cur_col: usize) {
        let window = &mut self.windows[window_idx];
        let viewport = window.viewport;
        if cur_row < window.rowoff { window.rowoff = cur_row; }
        if cur_row >= window.rowoff + viewport.height as usize { window.rowoff = cur_row - viewport.height as usize + 1; }
        if cur_col < window.coloff { window.coloff = cur_col; }
        if cur_col >= window.coloff + (viewport.width.saturating_sub(2)) as usize { window.coloff = cur_col - (viewport.width.saturating_sub(2)) as usize + 1; }
    }

    pub(crate) fn find_index_by_coords(&self, window_idx: usize, target_row: usize, target_col: usize) -> usize {
        let window = &self.windows[window_idx];
        let line_starts = &window.buffer.line_starts;
        if target_row >= line_starts.len() {
            return window.buffer.content().len();
        }
        let line_start = line_starts[target_row];
        let content = window.buffer.content();
        let line_content = if target_row + 1 < line_starts.len() {
            &content[line_start..line_starts[target_row + 1]]
        } else {
            &content[line_start..]
        };
        
        let mut byte_offset = 0;
        let mut char_count = 0;
        for c in line_content.chars() {
            if char_count >= target_col || c == '\n' {
                break;
            }
            byte_offset += c.len_utf8();
            char_count += 1;
        }
        line_start + byte_offset
    }
}
