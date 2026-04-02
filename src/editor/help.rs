use super::state::Editor;
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
            "  Ctrl-W : Switch Focus",
            "  Ctrl-C : Copy Selection",
            "  Ctrl-X : Cut Selection",
            "  Ctrl-V : Paste From Clipboard",
            "  Shift-Arrows: Select Text",
            "  Tab    : Indent Selection/Snippet",
            "  S-Tab  : Outdent Selection/Line",
            "  Arrows : Move Cursor",
            "  Ctrl + E : Enter DAli-Term Command Mode", "",
            "DAli-Term Commands:",
            "  h      : Toggle this Help",
            "  s      : Save File",
            "  q      : Exit Editor",
            "  vsplit : Split View (:vsplit term for terminal)",
            "  pwd    : Show Current Directory",
            "  ls     : List Files",
            "  tree   : Directory Tree",
            "  build  : Compile and Run", "",
            "Press ESC or q to return to editor.",
        ];

        let scroll = self.help_scroll;
        for i in 0..screen_rows {
            self.terminal.move_cursor(0, i as u16);
            if let Some(line) = help_text.get(i as usize + scroll) {
                if i == 0 && scroll == 0 { self.terminal.set_color_24bit(0, 255, 255); }
                else { self.terminal.set_color_24bit(0, 200, 200); }
                self.terminal.write_content(line);
                self.terminal.clear_from_cursor_to_end();
            } else {
                self.terminal.clear_line();
            }
        }
        self.draw_status_bar(screen_rows, screen_cols);
        self.draw_command_bar(screen_rows, screen_cols, 0, 0);
        self.terminal.flush()?;
        Ok(())
    }
}
