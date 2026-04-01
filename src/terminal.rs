use std::io::{self, Write};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, size};
use crossterm::ExecutableCommand;

pub struct AppendBuffer {
    b: Vec<u8>,
}

impl AppendBuffer {
    pub fn new() -> Self {
        Self { b: Vec::new() }
    }

    pub fn append(&mut self, s: &str) {
        self.b.extend_from_slice(s.as_bytes());
    }

    pub fn flush(&mut self, stdout: &mut io::Stdout) -> io::Result<()> {
        stdout.write_all(&self.b)?;
        stdout.flush()?;
        self.b.clear();
        Ok(())
    }
}

pub struct Terminal {
    stdout: io::Stdout,
    pub buffer: AppendBuffer,
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        let mut stdout = io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut term = Self {
            stdout,
            buffer: AppendBuffer::new(),
        };
        term.hide_cursor();
        term.clear_screen();
        term.flush()?;
        Ok(term)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush(&mut self.stdout)
    }

    pub fn size(&self) -> io::Result<(u16, u16)> {
        size()
    }

    pub fn move_cursor(&mut self, x: u16, y: u16) {
        self.buffer.append(&format!("\x1b[{};{}H", y + 1, x + 1));
    }

    pub fn clear_screen(&mut self) {
        self.buffer.append("\x1b[2J\x1b[H");
    }

    pub fn clear_line(&mut self) {
        self.buffer.append("\x1b[2K");
    }

    pub fn clear_from_cursor_to_end(&mut self) {
        self.buffer.append("\x1b[J");
    }

    pub fn hide_cursor(&mut self) {
        self.buffer.append("\x1b[?25l");
    }

    pub fn show_cursor(&mut self) {
        self.buffer.append("\x1b[?25h");
    }

    pub fn set_color_24bit(&mut self, r: u8, g: u8, b: u8) {
        self.buffer.append(&format!("\x1b[38;2;{};{};{}m", r, g, b));
    }

    pub fn reset_color(&mut self) {
        self.buffer.append("\x1b[0m");
    }

    pub fn write_content(&mut self, content: &str) {
        self.buffer.append(content);
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.show_cursor();
        self.reset_color();
        let _ = self.flush();
        let _ = disable_raw_mode();
        let _ = self.stdout.execute(LeaveAlternateScreen);
    }
}
