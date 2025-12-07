pub struct Editor {
    pub content: String,
    pub cursor_position: usize,
    pub scroll_offset: usize,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_position: 0,
            scroll_offset: 0,
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.cursor_position = self.content.len();
        self.scroll_offset = 0;
    }

    pub fn insert_char(&mut self, c: char) {
        let byte_pos = self.get_byte_position(self.cursor_position);
        self.content.insert(byte_pos, c);
        self.cursor_position += 1;
    }

    pub fn insert_newline(&mut self) {
        self.insert_char('\n');
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            let byte_pos = self.get_byte_position(self.cursor_position);
            self.content.remove(byte_pos);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        let char_count = self.content.chars().count();
        if self.cursor_position < char_count {
            self.cursor_position += 1;
        }
    }

    pub fn move_cursor_up(&mut self) {
        let lines: Vec<&str> = self.content.lines().collect();
        let (current_line, col) = self.get_cursor_line_col();

        if current_line > 0 {
            let prev_line_len = lines[current_line - 1].chars().count();
            let new_col = col.min(prev_line_len);

            let mut new_pos = 0;
            for i in 0..(current_line - 1) {
                new_pos += lines[i].chars().count() + 1;
            }
            new_pos += new_col;

            self.cursor_position = new_pos;
        }
    }

    pub fn move_cursor_down(&mut self) {
        let lines: Vec<&str> = self.content.lines().collect();
        let (current_line, col) = self.get_cursor_line_col();

        if current_line < lines.len() - 1 {
            let next_line_len = lines[current_line + 1].chars().count();
            let new_col = col.min(next_line_len);

            let mut new_pos = 0;
            for i in 0..=current_line {
                new_pos += lines[i].chars().count() + 1;
            }
            new_pos += new_col;

            self.cursor_position = new_pos;
        }
    }

    fn get_cursor_line_col(&self) -> (usize, usize) {
        let mut char_count = 0;
        let lines: Vec<&str> = self.content.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let line_len = line.chars().count();
            if char_count + line_len >= self.cursor_position {
                return (i, self.cursor_position - char_count);
            }
            char_count += line_len + 1;
        }

        (lines.len().saturating_sub(1), 0)
    }

    fn get_byte_position(&self, char_position: usize) -> usize {
        self.content
            .chars()
            .take(char_position)
            .map(|c| c.len_utf8())
            .sum()
    }

    pub fn get_display_lines(&self, height: usize) -> Vec<String> {
        let lines: Vec<String> = self.content.lines().map(|s| s.to_string()).collect();

        if lines.is_empty() {
            return vec![String::new(); height.min(1)];
        }

        if self.scroll_offset >= lines.len() {
            return vec![String::new(); height.min(1)];
        }

        let end = (self.scroll_offset + height).min(lines.len());
        let mut result: Vec<String> = lines[self.scroll_offset..end].to_vec();

        while result.len() < height {
            result.push(String::new());
        }

        result
    }

    pub fn adjust_scroll(&mut self, viewport_height: usize) {
        let (current_line, _) = self.get_cursor_line_col();

        if current_line < self.scroll_offset {
            self.scroll_offset = current_line;
        } else if current_line >= self.scroll_offset + viewport_height {
            self.scroll_offset = current_line - viewport_height + 1;
        }
    }
}
