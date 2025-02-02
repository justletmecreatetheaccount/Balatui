use crate::app::Message;

use crossterm::event::{KeyCode, KeyEvent};

pub struct EditApp {}

enum EditMode {
    Normal,
    Append,
    Visual,
}

impl EditApp {
    pub fn new() -> EditApp {
        EditApp {}
    }
    pub fn handle_key_press(&mut self, key: KeyEvent) -> Message {
        todo!()
    }
}
