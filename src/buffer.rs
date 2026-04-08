#[derive(Debug, Clone, Copy)]
pub enum EditAction {
    Insert(usize, char),
    Delete(usize, char),
}

pub struct GapBuffer {
    buffer: Vec<u8>,
    gap_start: usize,
    gap_end: usize,
    pub(crate) undo_stack: Vec<EditAction>,
    pub(crate) redo_stack: Vec<EditAction>,
    pub(crate) line_starts: Vec<usize>,
    pub revision_id: u64,
}

impl GapBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![b' '; capacity],
            gap_start: 0,
            gap_end: capacity,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            line_starts: vec![0],
            revision_id: 0,
        }
    }

    fn increment_revision(&mut self) {
        self.revision_id = self.revision_id.wrapping_add(1);
    }

    pub fn get_revision(&self) -> u64 {
        self.revision_id
    }

    fn grow(&mut self) {
        let old_capacity = self.buffer.len();
        let new_capacity = old_capacity * 2;
        let mut new_buffer = vec![b' '; new_capacity];

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
        let mut char_buf = [0; 4];
        let encoded = c.encode_utf8(&mut char_buf);
        let len = encoded.len();

        while self.gap_end - self.gap_start < len {
            self.grow();
        }

        self.undo_stack.push(EditAction::Insert(self.gap_start, c));
        self.redo_stack.clear();

        self.buffer[self.gap_start..self.gap_start + len].copy_from_slice(encoded.as_bytes());
        self.gap_start += len;
        self.update_line_starts();
        self.increment_revision();
    }

    pub fn delete(&mut self) -> Option<char> {
        if self.gap_start > 0 {
            let mut pos = self.gap_start - 1;
            while pos > 0 && (self.buffer[pos] & 0xC0) == 0x80 {
                pos -= 1;
            }

            let c_bytes = &self.buffer[pos..self.gap_start];
            let c_str = std::str::from_utf8(c_bytes).ok()?;
            let c = c_str.chars().next()?;

            self.undo_stack.push(EditAction::Delete(pos, c));
            self.redo_stack.clear();
            self.gap_start = pos;
            self.update_line_starts();
            self.increment_revision();
            Some(c)
        } else {
            None
        }
    }

    pub fn undo(&mut self) {
        if let Some(action) = self.undo_stack.pop() {
            let current_pos = self.gap_start;
            match action {
                EditAction::Insert(pos, c) => {
                    self.move_cursor(pos + c.len_utf8());
                    self.delete();
                    self.redo_stack.push(EditAction::Insert(pos, c));
                }
                EditAction::Delete(pos, c) => {
                    self.move_cursor(pos);
                    self.insert_no_undo(c);
                    self.redo_stack.push(EditAction::Delete(pos, c));
                }
            }
            self.move_cursor(current_pos.min(self.content().len()));
            self.update_line_starts();
            self.increment_revision();
        }
    }

    pub fn redo(&mut self) {
        if let Some(action) = self.redo_stack.pop() {
            match action {
                EditAction::Insert(pos, c) => {
                    self.move_cursor(pos);
                    self.insert_no_undo(c);
                    self.undo_stack.push(EditAction::Insert(pos, c));
                }
                EditAction::Delete(pos, c) => {
                    self.move_cursor(pos + c.len_utf8());
                    self.delete();
                    self.undo_stack.push(EditAction::Delete(pos, c));
                }
            }
            self.update_line_starts();
            self.increment_revision();
        }
    }
    
    fn insert_no_undo(&mut self, c: char) {
        let mut char_buf = [0; 4];
        let encoded = c.encode_utf8(&mut char_buf);
        let len = encoded.len();
        while self.gap_end - self.gap_start < len {
            self.grow();
        }
        self.buffer[self.gap_start..self.gap_start + len].copy_from_slice(encoded.as_bytes());
        self.gap_start += len;
    }

    pub fn update_line_starts(&mut self) {
        let mut starts = vec![0];
        let mut current_pos = 0;
        
        for i in 0..self.gap_start {
            if self.buffer[i] == b'\n' {
                starts.push(current_pos + 1);
            }
            current_pos += 1;
        }
        // Skip the gap
        let _gap_len = self.gap_end - self.gap_start;
        for i in self.gap_end..self.buffer.len() {
            if self.buffer[i] == b'\n' {
                starts.push(current_pos + 1);
            }
            current_pos += 1;
        }
        self.line_starts = starts;
    }

    pub fn move_left(&mut self) {
        if self.gap_start > 0 {
            let mut pos = self.gap_start - 1;
            while pos > 0 && (self.buffer[pos] & 0xC0) == 0x80 {
                pos -= 1;
            }
            self.move_cursor(pos);
        }
    }

    pub fn move_right(&mut self) {
        if self.gap_end < self.buffer.len() {
            let mut len = 1;
            while self.gap_end + len < self.buffer.len() && (self.buffer[self.gap_end + len] & 0xC0) == 0x80 {
                len += 1;
            }
            self.move_cursor(self.gap_start + len);
        }
    }

    pub fn move_cursor(&mut self, new_pos: usize) {
        if new_pos == self.gap_start {
            return;
        }

        if new_pos < self.gap_start {
            let distance = self.gap_start - new_pos;
            let src_start = new_pos;
            let src_end = self.gap_start;
            let dest_start = self.gap_end - distance;
            
            self.buffer.copy_within(src_start..src_end, dest_start);
            self.gap_start = new_pos;
            self.gap_end = dest_start;
        } else {
            let distance = new_pos - self.gap_start;
            let src_start = self.gap_end;
            let src_end = self.gap_end + distance;
            let dest_start = self.gap_start;
            
            self.buffer.copy_within(src_start..src_end, dest_start);
            self.gap_start = new_pos;
            self.gap_end = src_end;
        }
    }

    pub fn get_chunks(&self) -> (&[u8], &[u8]) {
        (&self.buffer[..self.gap_start], &self.buffer[self.gap_end..])
    }
    
    pub fn get_chunks_str(&self) -> (&str, &str) {
        let prefix = std::str::from_utf8(&self.buffer[..self.gap_start]).unwrap_or("");
        let suffix = std::str::from_utf8(&self.buffer[self.gap_end..]).unwrap_or("");
        (prefix, suffix)
    }

    pub fn content(&self) -> String {
        let mut s = String::with_capacity(self.buffer.len() - (self.gap_end - self.gap_start));
        s.push_str(std::str::from_utf8(&self.buffer[..self.gap_start]).unwrap_or(""));
        s.push_str(std::str::from_utf8(&self.buffer[self.gap_end..]).unwrap_or(""));
        s
    }

    pub fn cursor_pos(&self) -> usize {
        self.gap_start
    }
}
