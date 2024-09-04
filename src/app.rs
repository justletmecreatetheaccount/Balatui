use crossterm::event::{self, Event, KeyCode, KeyEvent};
use dirs;
use ratatui::{backend::Backend, layout::Margin, Terminal};
use std::{
    fs::{self, File, ReadDir},
    io::{self, Error, ErrorKind, Write},
    path::{Path, PathBuf},
};

use crate::ui::ui;

/*
The enum AppState defines states of the app.
Theses states define a state of operations an functions that the app
should perform at that time

MainScreen :
The first state that the user sees, can chose a deck
NewConfig :
Tell balatui to change it's config (ex. change deck location)
Editing :
Change card from the deck
Testing :
Test the user with the cards
*/
pub enum AppState {
    MainScreen,
    NewConfig,
    SelectCard,
    NewDeck, //TODO
    Editing, //TODO
    Testing, //TODO
}
/*
Defines the app sturcture :
AppState defines the state that the app is currently in
  */
pub struct App {
    pub state: AppState,
    exit: bool,

    config_path: PathBuf,
    decks_path: Option<String>,
    pub number_of_decks: usize,

    pub input_buffer: String,
    pub input_buffer_max_size: usize,
    pub cursor_position: usize,

    pub scrollbar_position: usize,
    pub selected_deck_index: usize,
    pub selected_card_index: usize,
}

impl App {
    //Creates a new instance of the app
    pub fn new() -> Result<App, io::Error> {
        let app: App;
        if let Some(mut home_dir) = dirs::config_dir() {
            home_dir.push("balatui");
            home_dir.push("balatui.conf");
            app = Self {
                state: AppState::MainScreen,
                exit: false,
                config_path: home_dir,
                decks_path: None,
                number_of_decks: 69,
                input_buffer: String::from(""),
                input_buffer_max_size: 0,
                cursor_position: 0,
                scrollbar_position: 0,
                selected_deck_index: 0,
                selected_card_index: 0,
            };
            Ok(app)
        } else {
            Err(Error::other("Can't find config dir"))
        }
    }

    //INPUT PARSING
    fn key_event_manager(&mut self, key_event: KeyEvent) -> io::Result<()> {
        match self.state {
            AppState::MainScreen => match key_event.code {
                KeyCode::Char('q') => {
                    self.exit();
                    Ok(())
                } //Exit app
                KeyCode::Char('n') => {
                    self.state = AppState::NewDeck;
                    Ok(())
                }
                KeyCode::Left => {
                    if self.selected_deck_index > 0 {
                        self.selected_deck_index -= 1;
                    }
                    Ok(())
                } //Navigate cards
                KeyCode::Right => {
                    if self.number_of_decks > 1
                        && self.selected_deck_index < self.number_of_decks - 1
                    {
                        self.selected_deck_index += 1;
                    }
                    Ok(())
                } //Navigate cards
                KeyCode::Down => {
                    if self.number_of_decks > 3
                        && self.selected_deck_index < self.number_of_decks - 3
                    {
                        self.selected_deck_index += 3;
                    }
                    Ok(())
                } //Navigate cards
                KeyCode::Up => {
                    if self.selected_deck_index > 2 {
                        self.selected_deck_index -= 3;
                    }
                    Ok(())
                } //Navigate cards
                KeyCode::Enter => {
                    self.state = AppState::SelectCard;
                    Ok(())
                }
                _ => Ok(()),
            },

            AppState::NewConfig => match key_event.code {
                KeyCode::Esc => {
                    self.exit();
                    Ok(())
                }
                KeyCode::Char(char) => {
                    if self.input_buffer.chars().count() < self.input_buffer_max_size as usize {
                        self.input_buffer.push(char);
                        self.cursor_position = self.cursor_position.saturating_add(1);
                    }
                    Ok(())
                }
                KeyCode::Delete => {
                    self.input_buffer.pop();
                    self.cursor_position = self.cursor_position.saturating_sub(1);
                    Ok(())
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                    self.cursor_position = self.cursor_position.saturating_sub(1);
                    Ok(())
                }
                KeyCode::Enter => {
                    self.create_new_config()?;
                    self.state = AppState::MainScreen;
                    self.input_buffer = String::from("");
                    self.input_buffer_max_size = 0;
                    Ok(())
                }
                _ => Ok(()),
            },
            AppState::Testing => match key_event.code {
                _ => Ok(()),
            },
            AppState::SelectCard => match key_event.code {
                KeyCode::Char('q') => {
                    self.state = AppState::MainScreen;
                    Ok(())
                } //Exit SelectCard Mode
                KeyCode::Left => {
                    if self.selected_card_index > 0 {
                        self.selected_card_index -= 1;
                    }
                    Ok(())
                } //Navigate cards
                KeyCode::Right => {
                    if self.selected_card_index < self.number_of_decks - 1 {
                        self.selected_card_index += 1;
                    }
                    Ok(())
                } //Navigate cards
                KeyCode::Down => {
                    if self.selected_card_index < self.number_of_decks - 3 {
                        self.selected_card_index += 3;
                    }
                    Ok(())
                } //Navigate cards
                KeyCode::Up => {
                    if self.selected_card_index > 2 {
                        self.selected_card_index -= 3;
                    }
                    Ok(())
                } //Navigate cards
                KeyCode::Enter => {
                    self.state = AppState::Editing;
                    Ok(())
                } //Select card for editing
                KeyCode::Char('n') => {
                    // Create New Card
                    self.state = AppState::Editing;
                    Ok(())
                }
                _ => Ok(()),
            },
            _ => todo!(),
        }
    }
    //INPUT PARSING

    //MAIN LOOP
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        //FIND CONFIG
        if self.config_path.exists() {
            let card_path_string = fs::read_to_string(&self.config_path)?;
            self.decks_path = Some(card_path_string);
        } else {
            self.state = AppState::NewConfig;
        }
        loop {
            if self.exit {
                break;
            }
            terminal.draw(|f| ui(f, self))?;
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

    fn create_new_deck(&mut self) -> io::Result<()> {
        if let Some(path) = &self.decks_path {
            let new_file = File::create(format!("{path}/{}", self.input_buffer));
        } else {
        }
        Ok(())
    }

    fn load_mainscreen_data(&mut self) {
        todo!();
    }

    fn edit_flashcards(&mut self) {
        //TOTOTOTODOOOOOOOOOOOOOOOOOOOOO
        todo!();
    }

    fn create_new_config(&mut self) -> io::Result<()> {
        let prefix = self.config_path.parent().unwrap();
        fs::create_dir_all(prefix)?;
        if let Ok(mut config_file) = File::create(&self.config_path) {
            config_file.write_all(self.input_buffer.as_bytes())?;
            fs::create_dir_all(PathBuf::from(&self.input_buffer))?;
            self.decks_path = Some(self.input_buffer.clone());
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                "Could not create config file",
            ))
        }
    }
}
