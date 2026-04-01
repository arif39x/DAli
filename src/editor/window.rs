use super::state::Editor;
use super::types::Rect;

impl Editor {
    pub(crate) fn recalculate_viewports(&mut self, cols: u16, rows: u16) {
        if self.windows.len() == 1 {
            self.windows[0].viewport = Rect { x: 0, y: 0, width: cols, height: rows };
        } else {
            let mid = cols / 2;
            self.windows[0].viewport = Rect { x: 0, y: 0, width: mid, height: rows };
            for i in 1..self.windows.len() {
                 self.windows[i].viewport = Rect { x: mid, y: 0, width: cols - mid, height: rows };
            }
        }
    }
}
