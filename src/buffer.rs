pub struct GapBuffer {
    buffer: Vec<char>,
    gap_start: usize,
    gap_end: usize,
}

impl GapBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![' '; capacity],
            gap_start: 0,
            gap_end: capacity,
        }
    }

    fn grow(&mut self) {
        let old_capacity = self.buffer.len();
        let new_capacity = old_capacity * 2;
        let mut new_buffer = vec![' '; new_capacity];

        // Copy prefix
        new_buffer[..self.gap_start].copy_from_slice(&self.buffer[..self.gap_start]);

        // Copy suffix to the end of the new buffer
        let suffix_len = old_capacity - self.gap_end;
        let new_gap_end = new_capacity - suffix_len;
        new_buffer[new_gap_end..].copy_from_slice(&self.buffer[self.gap_end..]);

        self.buffer = new_buffer;
        self.gap_end = new_gap_end;
    }

    pub fn insert(&mut self, c: char) {
        if self.gap_start == self.gap_end {
            self.grow();
        }
        self.buffer[self.gap_start] = c;
        self.gap_start += 1;
    }

    pub fn delete(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
        }
    }

    pub fn move_cursor(&mut self, new_pos: usize) {
        if new_pos == self.gap_start {
            return;
        }

        if new_pos < self.gap_start {
            // Shift gap to the left
            let distance = self.gap_start - new_pos;
            for _ in 0..distance {
                self.gap_start -= 1;
                self.gap_end -= 1;
                self.buffer[self.gap_end] = self.buffer[self.gap_start];
            }
        } else {
            // Shift gap to the right
            let distance = new_pos - self.gap_start;
            for _ in 0..distance {
                self.buffer[self.gap_start] = self.buffer[self.gap_end];
                self.gap_start += 1;
                self.gap_end += 1;
            }
        }
    }

    pub fn content(&self) -> String {
        let mut s = String::with_capacity(self.buffer.len() - (self.gap_end - self.gap_start));
        for i in 0..self.gap_start {
            s.push(self.buffer[i]);
        }
        for i in self.gap_end..self.buffer.len() {
            s.push(self.buffer[i]);
        }
        s
    }

    pub fn cursor_pos(&self) -> usize {
        self.gap_start
    }
}
