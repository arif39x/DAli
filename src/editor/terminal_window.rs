use super::state::Editor;
use std::error::Error;

impl Editor {
    pub(crate) fn draw_terminal_window(&mut self, index: usize, buffer: Vec<String>) -> Result<(), Box<dyn Error>> {
        let viewport = self.windows[index].viewport;
        self.terminal.set_color_24bit(0, 0, 0); 
        for r in 0..viewport.height {
            self.terminal.move_cursor(viewport.x, viewport.y + r);
            self.terminal.write_content(&" ".repeat(viewport.width as usize));
        }
        self.terminal.set_color_24bit(0, 255, 0);
        for (i, line) in buffer.iter().rev().enumerate() {
            if i >= viewport.height as usize { break; }
            self.terminal.move_cursor(viewport.x, viewport.y + viewport.height - 1 - i as u16);
            let truncated = if line.len() > viewport.width as usize { &line[..viewport.width as usize] } else { line };
            self.terminal.write_content(truncated);
        }
        self.terminal.reset_color();
        self.windows[index].dirty = false;
        Ok(())
    }
}
