use crate::modes::edit::EditApp;
use crate::modes::selection::SelectionApp;
use crate::modes::testing::TestingApp;
use crate::term;

use color_eyre::eyre::Context;
use color_eyre::eyre::Result;
use crossterm::event::{Event, KeyEventKind};
use ratatui::{prelude::*, widgets::*};
use std::io::Error;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

pub struct App {
    mode: Mode,
    selection_mode: SelectionApp,
    edit_mode: EditApp,
    testing_mode: TestingApp,
    cards_path: PathBuf,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    //The data structure that the app uses to decide wich mode it's in
    //The two selection modes share their implementations because of similarities
    SelectionDeck,
    SelectionCard,
    Edit,
    Testing,
    Quit,
}

#[derive(PartialEq, Eq)]
pub enum Message {
    //Data structure that modes use to comunicate to the main app actions
    ChangeMode(Mode),
    Delete(u32),
    Nothing,
}

impl App {
    pub fn new() -> Result<App> {
        let read_cards_path: Option<String> = None; //Read cards path form file
        Ok(App {
            mode: Mode::SelectionDeck,
            selection_mode: SelectionApp::new(),
            edit_mode: EditApp::new(),
            testing_mode: TestingApp::new(),
            cards_path: PathBuf::from_str("TEMPORARY").unwrap(), //To rework
        })
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        while self.is_running() {
            self.draw(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal
            .draw(|frame| frame.render_widget(self, frame.area()))
            .wrap_err("terminal.draw")?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        let message;
        match term::next_event(timeout)? {
            Some(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                //every mode handles it's own key presses
                match self.mode {
                    Mode::SelectionCard => {
                        message = self.selection_mode.handle_key_press(key, self.mode)
                    }
                    Mode::SelectionDeck => {
                        message = self.selection_mode.handle_key_press(key, self.mode)
                    }
                    Mode::Edit => message = self.edit_mode.handle_key_press(key),
                    Mode::Testing => message = self.testing_mode.handle_key_press(key),
                    Mode::Quit => message = Message::Nothing,
                }
            }
            _ => message = Message::Nothing,
        }
        match message {
            Message::ChangeMode(mode) => self.mode = mode,
            _ => {}
        }
        Ok(())
    }
}

/// Implement Widget for &App rather than for App as we would otherwise have to clone or copy the
/// entire app state on every frame.
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [title_bar, tab, bottom_bar] = vertical.areas(area);

        self.render_selected_mode(tab, buf);
        //self.render_bottom_bar(bottom_bar, buf);
    }
}

impl App {
    fn render_selected_mode(&self, area: Rect, buf: &mut Buffer) {
        match self.mode {
            Mode::SelectionDeck => self.selection_mode.render(area, buf),
            _ => {}
        }
    }

    fn render_bottom_bar(&self, area: Rect, buf: &mut Buffer) {
        todo!() //Here we will render controls with a specific "controls" data structure
    }
}
