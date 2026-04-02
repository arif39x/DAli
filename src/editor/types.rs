use crate::buffer::GapBuffer;

#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode {
    Editor,
    FileList,
    Help,
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

pub struct TerminalState {
    pub(crate) buffer: Vec<String>,
}

pub struct Window {
    pub(crate) buffer: GapBuffer,
    pub(crate) terminal_state: Option<TerminalState>,
    pub(crate) viewport: Rect,
    pub(crate) rowoff: usize,
    pub(crate) coloff: usize,
    pub(crate) filename: String,
    pub(crate) selection_start: Option<usize>,
    pub(crate) dirty: bool,
}
