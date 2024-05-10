use std::{io::{self, Error, ErrorKind, Write},
    path::{PathBuf, Path},
    fs::{self, ReadDir, File}};
use dirs;
use crossterm::event::{self,
    Event, KeyCode,
    KeyEvent};
use ratatui::{backend::Backend, Terminal};

use crate::ui::ui;

pub enum AppState {
    MainScreen,
    NewConfig,
    Editing, //TODO MAYBE
    Testing,
}

pub struct App {
    pub state : AppState,
    exit : bool,

    config_path : PathBuf,
    cards_path : Option<ReadDir>,

    pub input_buffer : String,
    pub input_buffer_max_size : u16,
    pub cursor_position : u16,
}

impl App {

    pub fn new() -> Result<App, io::Error> {
        let app : App;
        if let Some(mut home_dir) = dirs::config_dir() {
            home_dir.push("balatui");
            home_dir.push("balatui.conf");
            app = Self {
            state : AppState::MainScreen,
            exit : false,
            config_path : home_dir,
            cards_path : None,
            input_buffer : String::from(""),
            input_buffer_max_size : 0,
            cursor_position : 0,
            };
            Ok(app)
        } else {
            Err(Error::other("Can't find config dir"))
        }
    }

    //INPUT PARSING
    fn key_event_manager(&mut self, key_event : KeyEvent) -> io::Result<()> {
        match self.state {
            AppState::MainScreen =>
                match key_event.code {
                    KeyCode::Char('q') => {self.exit(); Ok(())},
                    KeyCode::Char('n') => todo!(),
                    KeyCode::Left => todo!(),
                    KeyCode::Right => todo!(),
                    KeyCode::Down => todo!(),
                    KeyCode::Up => todo!(),
                    _ => {Ok(())}
                },
            AppState::NewConfig => {
                match key_event.code {
                    KeyCode::Esc => {self.exit(); Ok(())},
                    KeyCode::Char(char) => {
                        if self.input_buffer.chars().count() < self.input_buffer_max_size as usize { 
                        self.input_buffer.push(char);
                        self.cursor_position = self.cursor_position.saturating_add(1);
                        }
                        Ok(())
                        },
                    KeyCode::Delete => {
                        self.input_buffer.pop();
                        self.cursor_position = self.cursor_position.saturating_sub(1);
                        Ok(())},
                    KeyCode::Backspace => {
                        self.input_buffer.pop();
                        self.cursor_position = self.cursor_position.saturating_sub(1);
                        Ok(())},
                    KeyCode::Enter => {
                        self.create_new_config()?;
                        self.state = AppState::MainScreen;
                        self.input_buffer = String::from("");
                        self.input_buffer_max_size = 0;
                        Ok(())
                    },
                    _ => {Ok(())},
                }
            },
            _ => todo!(),
        }
    }
    //INPUT PARSING
    //MAIN LOOP
    pub fn run<B : Backend>(&mut self, terminal : &mut Terminal<B>) -> io::Result<()> {
        //FIND CONFIG
        if self.config_path.exists() {
            let card_path_string = fs::read_to_string(&self.config_path)?;
            self.cards_path = Some(fs::read_dir(card_path_string)?);
        } else {
            self.state = AppState::NewConfig;
        }
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
                self.key_event_manager(key_event)?
            }
        }
        Ok(())
    }
    //MAIN LOOP

    fn exit(&mut self) {
        self.exit = true;
    }

    fn edit_flashcards(&mut self) { //TOTOTOTODOOOOOOOOOOOOOOOOOOOOO
        self.cards_path = Some(fs::read_dir("./").unwrap());
        todo!();
    }

    fn create_new_config(&mut self) -> io::Result<()> {
        let prefix = self.config_path.parent().unwrap();
        fs::create_dir_all(prefix)?;
        if let Ok(mut config_file) = File::create(&self.config_path) {
            config_file.write_all(self.input_buffer.as_bytes())?;
            fs::create_dir_all(PathBuf::from(&self.input_buffer))?;
            self.cards_path = Some(fs::read_dir(&self.input_buffer)?);
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NotFound, "Could not create config file"))
        }
    }
}
