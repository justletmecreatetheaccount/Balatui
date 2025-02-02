use std::{collections::VecDeque, vec};

use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Paragraph, Widget},
};

pub struct Popup<T> {
    cursor_position: u16,
    pub number_of_buttons: u16,
    buttons_content: Vec<String>,
    popup_content: String,
    pub popup_type: T,
}

impl<T> Popup<T> {
    pub fn make_centered_rectangle_area(
        width_percentage: u32,
        height_percentage: u32,
        area: Rect,
    ) -> Rect {
        // Give the width_percentage and length in percentages of the given area
        Rect {
            x: area.x + area.width * (100 - width_percentage) as u16 / 200,
            y: area.y + area.height * (100 - height_percentage) as u16 / 200,
            width: area.width * width_percentage as u16 / 100,
            height: area.height * height_percentage as u16 / 100,
        }
    }

    pub fn new(popup_type: T, popup_content: String, buttons_content: Vec<String>) -> Popup<T> {
        Popup {
            cursor_position: 0,
            number_of_buttons: buttons_content.len() as u16,
            buttons_content,
            popup_content,
            popup_type,
        }
    }
    fn mv_left(&mut self) {
        if self.cursor_position > 0 && self.number_of_buttons != 0 {
            self.cursor_position = self.cursor_position - 1;
        }
    }

    fn mv_right(&mut self) {
        if self.cursor_position < self.number_of_buttons - 1 && self.number_of_buttons != 0 {
            self.cursor_position = self.cursor_position + 1;
        }
    }

    pub fn handle_key_press(&mut self, key: KeyEvent) -> u16 {
        use KeyCode::*;
        match key.code {
            Char('e') | Enter => return self.cursor_position,
            Char('h') | Left => self.mv_left(),
            Char('l') | Right => self.mv_right(),
            Char('k') | Up => self.mv_right(),
            Char('j') | Down => self.mv_left(),
            _ => {}
        };
        self.number_of_buttons
    }
}

impl<T> Widget for &Popup<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_block = Block::bordered(); //The box of the whole popup

        let vertical_layout = Layout::default() //Layout for a big paragrapg on top of the buttons
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(popup_block.inner(area));

        let horizontal_layout = Layout::default() //Layout on the bottom part for the different buttons
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(
                    (100 / self.number_of_buttons) as u16
                );
                self.number_of_buttons as usize
            ])
            .split(vertical_layout[1]);

        let popup_content = Paragraph::new(self.popup_content.as_str());
        popup_block.render(area, buf);
        popup_content.render(vertical_layout[0], buf);

        for n in 0..self.number_of_buttons {
            let button;
            if n == self.cursor_position {
                button = Paragraph::new(self.buttons_content[n as usize].as_str())
                    .block(Block::bordered().style(Style::new().red()));
            } else {
                button = Paragraph::new(self.buttons_content[n as usize].as_str())
                    .block(Block::bordered());
            }
            button.render(horizontal_layout[n as usize], buf);
        }
    }
}
