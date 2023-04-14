use std::rc::Rc;
use std::cell::RefCell;
use crate::mode::Mode;
use crate::revi::ReVi;
use std::fmt;

#[macro_export]
macro_rules! commands {
    ( $( $x:ident $(($($args:expr),*))? ),* ) => {
        vec![$(BoxedCommand { command: std::rc::Rc::new($x $(($($args),*))?) }),*]
    }

}

macro_rules! build_command {
    ($name:ident, $counter:expr $(, $ty:ty)?; $caller:expr) => {
        #[derive(Debug, PartialEq)]
        pub struct $name $((pub $ty))?;
        impl Command for $name {
            fn call(&self, revi_rc: Rc<RefCell<ReVi>>, count: usize, lua: &mlua::Lua) {
                $caller(&self, revi_rc, count, lua);
            }
            fn id(&self) -> usize {
                $counter
            }
        }
        impl From<$name> for BoxedCommand {
            fn from(value: $name) -> Self {
                Self {
                    command: std::rc::Rc::new(value),
                }
            }
        }
    };
}

pub trait Command: fmt::Debug {
    fn call(&self, revi_rc: Rc<RefCell<ReVi>>, count: usize, lua: &mlua::Lua);
    fn id(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct BoxedCommand {
    pub command: Rc<dyn Command>,
}

impl mlua::UserData for BoxedCommand {}

impl BoxedCommand {
    pub fn new(command: impl Command + 'static) -> Self {
        Self {
            command: Rc::new(command),
        }
    }
}

impl PartialEq for BoxedCommand {
    fn eq(&self, other: &Self) -> bool {
        self.command.id() == other.command.id()
    }
}

build_command!(
    CursorUp,
    0;
    |_: &CursorUp, revi_rc: Rc<RefCell<ReVi>>, count: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().move_cursor_up(count);
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);
build_command!(
    CursorDown,
    1;
    |_: &CursorDown, revi_rc: Rc<RefCell<ReVi>>, count: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().move_cursor_down(count);
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);
build_command!(
    CursorLeft,
    2;
    |_: &CursorLeft, revi_rc: Rc<RefCell<ReVi>>, count: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().move_cursor_left(count);
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);
build_command!(
    CursorRight,
    3;
    |_: &CursorRight, revi_rc: Rc<RefCell<ReVi>>, count: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().move_cursor_right(count);
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);
build_command!(
    ScrollUp,
    4;
    |_: &ScrollUp, revi_rc: Rc<RefCell<ReVi>>, count: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().scroll_up(count);
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);
build_command!(
    ScrollDown,
    5;
    |_: &ScrollDown, revi_rc: Rc<RefCell<ReVi>>, count: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().scroll_up(count);
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);
build_command!(
    Home,
    6;
    |_: &Home, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().home();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);
build_command!(
    End,
    7;
    |_: &End, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().end();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);
build_command!(
    MoveForwardByWord,
    8;
    |_: &MoveForwardByWord, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().move_forward_by_word();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    MoveBackwardByWord,
    9;
    |_: &MoveBackwardByWord, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().move_backward_by_word();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    JumpToFirstLineBuffer,
    10;
    |_: &JumpToFirstLineBuffer, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().jump_to_first_line_buffer();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    JumpToLastLineBuffer,
    11;
    |_: &JumpToLastLineBuffer, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().jump_to_last_line_buffer();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    Backspace,
    12;
    |_: &Backspace, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().backspace();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    NewLine,
    13;
    |_: &NewLine, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().insert_newline();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    FirstCharInLine,
    14;
    |_: &FirstCharInLine, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().first_char_in_line();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    DeleteChar,
    15;
    |_: &DeleteChar, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().delete();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    DeleteLine,
    16;
    |_: &DeleteLine, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        let line = revi.focused_window_mut().delete_line();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
        revi.clipboard.clear();
        revi.clipboard.push_str(line.as_str());
    }
);

build_command!(
    YankLine,
    17;
    |_: &YankLine, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        let yanked_line;
        {
            let cursor = revi.focused_window().cursor_file();
            let line = cursor.as_usize_y();
            let buffer = revi.focused_window().buffer();
            yanked_line = buffer.line(line);
        }
        revi.clipboard.clear();
        revi.clipboard.push_str(yanked_line.as_str());
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    Paste,
    18;
    |_: &Paste, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
        // TODO: Fix this cloning.
        let clipboard = revi.clipboard.clone();
        {
            let window = revi.focused_window_mut();
            let line_idx = window.cursor_file().as_usize_y();
            let mut buffer = window.buffer_mut();
            buffer.insert_line(line_idx + 1, &clipboard);
        }
        revi.focused_window_mut().move_cursor_down(1);
    }
);

build_command!(
    PasteBack,
    19;
    |_: &PasteBack, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
        // TODO: Fix this cloning.
        let clipboard = revi.clipboard.clone();
        {
            let window = revi.focused_window_mut();
            let line_idx = window.cursor_file().as_usize_y();
            let mut buffer = window.buffer_mut();
            buffer.insert_line(line_idx + 1, &clipboard);
        }
    }
);

build_command!(
    InsertChar,
    20,
    char;
    |insert_char: &InsertChar, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window_mut().insert_char(insert_char.0);
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    ChangeMode,
    21,
    Mode;
    |change_mode: &ChangeMode, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.change_modes(change_mode.0);
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    EnterCommandMode,
    22;
    |_: &EnterCommandMode, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.enter_command_mode();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    ExitCommandMode,
    23;
    |_: &ExitCommandMode, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        if revi.focused == 0 {
            revi.exit_command_mode();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
        }
    }
);

build_command!(
    ExcuteCommandLine,
    24;
    |_: &ExcuteCommandLine, revi_rc: Rc<RefCell<ReVi>>, _: usize, lua: &mlua::Lua| {
        let line = {
            let mut revi = revi_rc.borrow_mut();
            if revi.focused != 0 {
                return;
            }
            let window = revi.get_command_window_mut();
            let mut line = window.get_current_line();
            if !line.is_empty() {
                line.remove(0);
            }
            line
        };
        // run lua code
        if line.starts_with("lua") {
            let Some((_, line)) = line.split_once(' ') else {
                revi_rc
                    .borrow_mut()
                    .error_message(&[line.as_str(), "lua command takes an argument expr"]);
                return;
            };
            let result = lua.load(line.trim()).exec();
            if let Err(e) = result {
                revi_rc.borrow_mut().output_message(e.to_string());
            }
            return;
        }
        // built in command
        revi_rc.borrow_mut().run_command_line(&line);
    }
);

build_command!(
    NextWindow,
    25;
    |_: &NextWindow, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.next_window();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    Print,
    26,
    String;
    |p: &Print, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.print(&p.0);
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    Save,
    27;
    |_: &Save, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.focused_window().save();
        let focused_window = revi.focused;
        revi.queue.push(focused_window);
    }
);

build_command!(
    Quit,
    28;
    |_: &Quit, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.exit();
    }
);

build_command!(
    CloseWindow,
    29;
    |_: &CloseWindow, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.close_current_window();
    }
);

build_command!(
    ListBuffers,
    30;
    |_: &ListBuffers, revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        revi.list_buffers();
    }
);

build_command!(
    InsertTab,
    31;
    |_: &InsertTab, revi_rc: Rc<RefCell<ReVi>>, count: usize, _: &mlua::Lua| {
        let mut revi = revi_rc.borrow_mut();
        for _ in 0..revi.settings.tab_width+count {
            revi.focused_window_mut().insert_char(' ');
        }
    }
);

build_command!(
    JumpListBack,
    32;
    |_: &JumpListBack, _revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        unimplemented!("JumpListBack");
    }
);

build_command!(
    JumpListForward,
    33;
    |_: &JumpListForward, _revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        unimplemented!("JumpListForward");
    }
);

build_command!(
    Undo,
    34;
    |_: &Undo, _revi_rc: Rc<RefCell<ReVi>>, _: usize, _: &mlua::Lua| {
        unimplemented!("Undo");
    }
);

