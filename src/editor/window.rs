use super::state::Editor;
use super::types::Rect;

impl Editor {
    pub(crate) fn recalculate_viewports(&mut self, cols: u16, rows: u16) {
        let num_windows = self.windows.len();
        if num_windows == 0 { return; }
        
        let pane_width = cols / num_windows as u16;
        for i in 0..num_windows {
            let x = i as u16 * pane_width;
            let width = if i == num_windows - 1 {
                cols - x
            } else {
                pane_width
            };
            self.windows[i].viewport = Rect { x, y: 0, width, height: rows };
            self.windows[i].dirty = true;
        }
    }
}
