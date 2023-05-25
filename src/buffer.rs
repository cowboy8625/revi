use revi_ui::layout::Pos;
use ropey::{Rope, RopeSlice};

#[derive(Debug, Clone, Copy, Default)]
pub struct Cursor {
    pub pos: Pos,
    pub scroll: Pos,
    max: Pos,
}

impl Cursor {
    pub fn up(&mut self, line_len: usize) {
        let max = self.max.x as usize;
        let col = self.pos.x as usize;
        let col = col.max(max).min(line_len);
        self.set_col(col);
        self.sub_row(1);
    }

    pub fn down(&mut self, height: usize) {
        let max = self.max.x as usize;
        let col = self.pos.x as usize;
        let col = col.max(max).min(height);
        self.set_col(col);
        self.add_row(1);
    }

    pub fn left(&mut self) {
        self.sub_col_effect_max(1);
    }

    pub fn right(&mut self) {
        self.add_col_effect_max(1);
    }

    pub fn scroll_up(&mut self) {
        self.scroll.y = self.scroll.y.saturating_sub(1);
    }

    pub fn scroll_down(&mut self, max: usize) {
        let max = max as u16;
        self.scroll.y = self.scroll.y.saturating_add(1).min(max);
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

    pub fn row_scroll(&self) -> usize {
        (self.pos.y + self.scroll.y) as usize
    }

    // pub fn row(&self) -> usize {
    //     self.pos.y as usize
    // }

    // pub fn add_row_effect_max(&mut self, row: usize) {
    //     let row = row as u16;
    //     self.pos.y = self.pos.y.saturating_add(row);
    //     self.max.y = self.pos.y.max(self.max.y);
    // }

    pub fn add_row(&mut self, row: usize) {
        let row = row as u16;
        self.pos.y = self.pos.y.saturating_add(row);
    }

    pub fn sub_row(&mut self, row: usize) {
        let row = row as u16;
        self.pos.y = self.pos.y.saturating_sub(row);
    }

    // pub fn sub_row_effect_max(&mut self, row: usize) {
    //     let row = row as u16;
    //     self.pos.y = self.pos.y.saturating_sub(row);
    //     self.max.y = self.pos.y.min(self.max.y);
    // }

    // pub fn col(&self) -> usize {
    //     self.pos.x as usize
    // }

    pub fn add_col_effect_max(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = self.pos.x.saturating_add(col);
        self.max.x = self.pos.x.max(self.max.x);
    }

    pub fn add_col(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = self.pos.x.saturating_add(col);
    }

    pub fn sub_col_effect_max(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = self.pos.x.saturating_sub(col);
        self.max.x = self.pos.x;
    }

    pub fn sub_col(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = self.pos.x.saturating_sub(col);
        self.max.x = self.pos.x;
    }

    pub fn set_col_effect(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = col;
        self.max.x = self.pos.x.min(self.max.x);
    }

    pub fn set_col(&mut self, col: usize) {
        let col = col as u16;
        self.pos.x = col;
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
        let col = self.cursor.pos.x as usize;
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
        let row = self.cursor.pos.y as usize;
        self.rope.line(row).len_chars().saturating_sub(2)
    }

    pub fn line_len(&self, row: usize) -> usize {
        self.rope.line(row).len_chars().saturating_sub(2)
    }

    pub fn get_cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn _get_cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    pub fn on_screen(&self, height: u16) -> Vec<RopeSlice> {
        let top = self.cursor.scroll.y as usize;
        let bottom = ((self.cursor.scroll.y + height) as usize).saturating_sub(top);
        self.rope.lines().skip(top).take(bottom).collect()
        // let mut result = vec![];
        // for idx in top..=bottom {
        //     let Some(line) = self.rope.get_line(idx) else {
        //         break;
        //     };
        //     result.push(line);
        // }
        // result
    }

    pub fn insert(&mut self, text: impl Into<String>) {
        let text = text.into();
        let row = self.cursor.pos.y as usize;
        let col = self.cursor.pos.x as usize;
        let char_idx = self.rope.line_to_char(row);
        self.rope.insert(char_idx + col, &text);
        let col = text.len();
        self.cursor.add_col(col);
        if text.contains('\n') {
            self.cursor.new_line();
        }
    }

    pub fn backspace(&mut self) {
        let col = self.cursor.pos.x as usize;
        let row = self.cursor.pos.y as usize;
        let char_idx = self.rope.line_to_char(row);
        let start = (char_idx + col).saturating_sub(1);
        let end = char_idx + col;
        self.rope.remove(start..end);
        if col == 0 {
            self.cursor_up();
            self.cursor_end();
            return;
        }
        self.cursor.left();
    }

    pub fn cursor_up(&mut self) -> bool {
        let row = self.cursor.pos.y as usize;
        if row == 0 {
            return false;
        }
        let row = row.saturating_sub(1);
        let len = self.line_len(row);
        self.cursor.up(len);
        true
    }

    pub fn cursor_down(&mut self, max: usize) -> bool {
        let row = self.cursor.pos.y as usize;
        if row >= max {
            return false;
        }
        let row = row.saturating_sub(1);
        let len = self.line_len(row);
        self.cursor.down(len);
        true
    }

    pub fn cursor_left(&mut self) {
        self.cursor.left();
    }

    pub fn cursor_right(&mut self) {
        let len_col = self.current_line_len();
        let row = self.cursor.pos.y as usize;
        if row < len_col {
            self.cursor.right();
        }
    }

    pub fn cursor_end(&mut self) {
        let row = self.cursor.pos.y as usize;
        let len = self.line_len(row);
        self.cursor.set_col(len);
    }

    pub fn cursor_home(&mut self) {
        self.cursor.set_col(0);
    }

    pub fn scroll_up(&mut self) {
        self.cursor.scroll_up();
    }

    pub fn scroll_down(&mut self) {
        let max = self.rope.lines().count();
        self.cursor.scroll_down(max);
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
