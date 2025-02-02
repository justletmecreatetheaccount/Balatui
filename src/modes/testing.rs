use crate::app::Message;

use crossterm::event::{KeyCode, KeyEvent};

pub struct TestingApp {}

impl TestingApp {
    pub fn new() -> TestingApp {
        TestingApp {}
    }
    pub fn handle_key_press(&mut self, key: KeyEvent) -> Message {
        todo!()
    }
}
