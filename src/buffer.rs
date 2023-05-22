use revi_ui::layout::Pos;
use ropey::{Rope, RopeSlice};

#[derive(Debug, Clone, Copy, Default)]
pub struct Cursor {
    pos: Pos,
    scroll: Pos,
    max: Pos,
}

impl Cursor {
    pub fn up(&mut self) {
        self.sub_row(1);
    }

    pub fn down(&mut self) {
        self.add_row(1);
    }

    pub fn left(&mut self) {
        self.sub_col(1);
    }

    pub fn right(&mut self) {
        self.add_col(1);
    }

    pub fn pos(&self) -> Pos {
        let x = self.pos.x + self.scroll.x;
        let y = self.pos.y + self.scroll.y;
        Pos { x, y }
    }

    pub fn new_line(&mut self) {
        self.set_col(0);
        self.add_row(1);
    }

    pub fn row(&self) -> usize {
        (self.pos.y + self.scroll.y) as usize
    }

    pub fn add_row(&mut self, row: usize) {
        let row = row as u16;
        self.pos.y = self.pos.y.saturating_add(row);
        self.max.y = self.pos.y.max(self.max.y);
    }
    pub fn sub_row(&mut self, row: usize) {
        let row = row as u16;
        self.pos.y = self.pos.y.saturating_sub(row);
        self.max.y = self.pos.y.min(self.max.y);
    }

    pub fn col(&self) -> usize {
        (self.pos.x + self.scroll.x) as usize
    }

    pub fn add_col(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = self.pos.x.saturating_add(col);
        self.max.x = self.pos.x.max(self.max.x);
    }
    pub fn sub_col(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = self.pos.x.saturating_sub(col);
        self.max.x = self.pos.x.min(self.max.x);
    }
    pub fn set_col(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = col;
        self.max.x = self.pos.x.min(self.max.x);
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub name: String,
    rope: Rope,
    cursor: Cursor,
}

impl Buffer {
    pub fn from_path(path: &str) -> Self {
        let src = std::fs::read_to_string(path).unwrap_or_default();
        Self {
            name: path.into(),
            rope: Rope::from_str(&src),
            cursor: Cursor::default(),
        }
    }

    pub fn align_cursor(&mut self) {
        let col = self.cursor.col();
        let max = self.current_line_len();
        if col < max {
            return;
        }
        self.cursor.set_col(max);
    }

    pub fn _get_line_len(&self, row: usize) -> Option<usize> {
        self.rope
            .get_line(row)
            .map(|line| line.len_chars().saturating_sub(1))
    }

    pub fn current_line_len(&self) -> usize {
        let row = self.cursor.row();
        self.rope.line(row).len_chars().saturating_sub(1)
    }

    pub fn get_cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn _get_cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    pub fn on_screen(&self, height: u16) -> Vec<RopeSlice> {
        let top = self.cursor.scroll.y as usize;
        let bottom = (self.cursor.scroll.y + height) as usize;
        let mut result = vec![];
        for idx in top..=bottom {
            let Some(line) = self.rope.get_line(idx) else {
                break;
            };
            result.push(line);
        }
        result
    }

    pub fn insert(&mut self, text: impl Into<String>) {
        let text = text.into();
        let row = self.cursor.row();
        let col = self.cursor.col();
        let char_idx = self.rope.line_to_char(row);
        self.rope.insert(char_idx + col, &text);
        let col = text.len();
        self.cursor.add_col(col);
        if text.contains('\n') {
            self.cursor.new_line();
        }
    }

    pub fn backspace(&mut self) {
        let col = self.cursor.col();
        let row = self.cursor.row();
        let char_idx = self.rope.line_to_char(row);
        let start = (char_idx + col).saturating_sub(1);
        let end = char_idx + col;
        self.rope.remove(start..end);
        if col == 0 {
            self.cursor.up();
            self.cursor_end();
            return;
        }
        self.cursor.left();
    }

    pub fn cursor_up(&mut self) {
        self.cursor.up();
    }

    pub fn cursor_down(&mut self) {
        let len_lines = self.rope.len_lines().saturating_sub(1);
        if self.cursor.row() < len_lines {
            self.cursor.down();
        }
    }

    pub fn cursor_left(&mut self) {
        self.cursor.left();
    }

    pub fn cursor_right(&mut self) {
        let len_col = self.current_line_len();
        if self.cursor.col() < len_col {
            self.cursor.right();
        }
    }

    pub fn cursor_end(&mut self) {
        let row = self.cursor.row();
        let len = self.rope.line(row).len_chars();
        self.cursor.set_col(len);
    }

    pub fn cursor_home(&mut self) {
        self.cursor.set_col(0);
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            name: "N/A".into(),
            rope: Rope::default(),
            cursor: Cursor::default(),
        }
    }
}
