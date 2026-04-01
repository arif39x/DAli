use crate::editor::Editor;
use std::error::Error;

impl Editor {
    pub(crate) fn draw_help_overlay(&mut self, screen_rows: u16, screen_cols: u16) -> Result<(), Box<dyn Error>> {
        self.terminal.move_cursor(0, 0);
        self.terminal.set_color_24bit(0, 200, 200);
        let help_text = vec![
            "DAli Editor Help ", "",
            "Editor Keys:",
            "  Ctrl-Q : Quit",
            "  Ctrl-F : Incremental Search",
            "  Ctrl-P : Fuzzy File Discovery",
            "  Ctrl-S : Quick Save",
            "  Arrows : Move Cursor",
            "  :      : Enter DAli-Term", "",
            "DAli-Term Commands:",
            "  h      : Toggle this Help",
            "  s      : Save File",
            "  q      : Exit Editor",
            "  build  : Compile and Run", "",
            "Press ESC, ENTER or q to return to editor.",
        ];
        for (i, line) in help_text.iter().enumerate() {
            if (i as u16) < screen_rows {
                self.terminal.move_cursor(0, i as u16);
                self.terminal.write_content(line);
                self.terminal.clear_from_cursor_to_end();
            }
        }
        for i in (help_text.len() as u16)..screen_rows {
            self.terminal.move_cursor(0, i);
            self.terminal.clear_line();
        }
        self.draw_status_bar(screen_rows, screen_cols, 0, 0);
        self.draw_command_bar(screen_rows, screen_cols, 0, 0);
        self.terminal.flush()?;
        Ok(())
    }
}
