use std::{io::{self, Error},
    path::PathBuf,
    fs::{self, ReadDir}};
use dirs;
use crossterm::event::{self,
    Event, KeyCode,
    KeyEvent,
    KeyEventKind};
use ratatui::{backend::Backend, style::palette::tailwind::SLATE, Terminal};

use crate::ui::ui;

pub enum AppState {
    MainScreen,
    NewConfig,
    Editing,
    Testing,
}

pub struct App {
    pub state : AppState,
    exit : bool,
    cards_paths : PathBuf,
    cards_files : Option<ReadDir>,
    editing_buffer : String,
}

impl App {

    pub fn new() -> Result<App, io::Error> {
        let app : App;
        if let Some(mut home_dir) = dirs::config_dir() {
            home_dir.push("balatui");
            home_dir.push("balatui.conf");
            app = Self { state : AppState::MainScreen,
            exit : false,
            cards_paths : home_dir,
            cards_files : None,
            editing_buffer : String::from(""),
            };
            Ok(app)
        } else {
            Err(Error::other("Can't find config dir"))
        }
    }

    //INPUT PARSING
    fn key_event_manager(&mut self, key_event : KeyEvent) {
        match self.state {
            AppState::MainScreen =>
                match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Char('n') => todo!(),
                    KeyCode::Left => todo!(),
                    KeyCode::Right => todo!(),
                    KeyCode::Down => todo!(),
                    KeyCode::Up => todo!(),
                    _ => {}
                },
            AppState::NewConfig => {
                match key_event.code {
                    KeyCode::Char(char) => {self.editing_buffer.push(char);},
                    KeyCode::Delete => {self.editing_buffer.pop();},
                    KeyCode::Backspace => {self.editing_buffer.pop();},
                    KeyCode::Enter => self.create_new_config(),
                    _ => {},
                }
            },
            _ => todo!(),
        }
    }
    //INPUT PARSING
    //MAIN LOOP
    pub fn run<B : Backend>(&mut self, terminal : &mut Terminal<B>) -> io::Result<()> {
        //FIND CONFIG
        //FIND CONFIG
        self.cards_files = match fs::read_dir(&self.cards_paths) {
            Ok(res) => Some(res),
            Err(_) => { 
                self.state = AppState::NewConfig;
                None },
        };
        loop {
            if self.exit {
                break;
            }
            terminal.draw(|f| ui(f, self, 8))?;
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    continue;
                }
                self.key_event_manager(key_event)
            }
        }
        Ok(())
    }
    //MAIN LOOP

    fn exit(&mut self) {
        self.exit = true;
    }

    fn edit_flashcards(&mut self) {
        todo!();
        self.cards_files = Some(fs::read_dir("./").unwrap());
    }

    fn create_new_config(&mut self) {
        todo!();
    }
}
