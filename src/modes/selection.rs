use std::{cmp::min_by, str::FromStr};

use crate::{
    app::{Message, Mode},
    popup::Popup,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Color,
    widgets::{
        Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
        Widget,
    },
};

enum SelectionPopupTypes {
    DeleteItem,
}

pub struct SelectionApp {
    cursor_position: u32,
    number_of_elements: u32,
    n_columns: u32, // Only cosmetic
    n_lines: u32,   // Only cosmetic
    elements: Vec<String>,
    current_popup: Option<Popup<SelectionPopupTypes>>,
}

impl SelectionApp {
    pub fn new() -> SelectionApp {
        SelectionApp {
            cursor_position: 0,
            number_of_elements: 13,
            n_columns: 3,
            n_lines: 3,
            elements: {
                // Needs rework obviously
                let mut temp: Vec<String> = vec![];
                for i in 0..13 {
                    temp.push(format!("number : {}", i));
                }
                temp
            },
            current_popup: None,
        }
    }

    fn mv_left(&mut self) {
        if self.cursor_position > 0 && self.number_of_elements != 0 {
            self.cursor_position = self.cursor_position - 1;
        }
    }

    fn mv_right(&mut self) {
        if self.cursor_position < self.number_of_elements - 1 && self.number_of_elements != 0 {
            self.cursor_position = self.cursor_position + 1;
        }
    }

    fn mv_up(&mut self) {
        if self.cursor_position > self.n_columns - 1 && self.number_of_elements != 0 {
            self.cursor_position = self.cursor_position - self.n_columns;
        }
    }

    fn mv_down(&mut self) {
        if self.cursor_position < self.number_of_elements - self.n_columns {
            self.cursor_position = self.cursor_position + self.n_columns;
        }
    }

    pub fn handle_key_press(&mut self, key: KeyEvent, current_mode: Mode) -> Message {
        use KeyCode::*;
        if self.current_popup.is_none() {
            // Check if a popup is active
            match key.code {
                Char('e') | Enter => {
                    if current_mode == Mode::SelectionDeck {
                        return Message::ChangeMode(Mode::SelectionCard);
                    } else if current_mode == Mode::SelectionCard {
                        return Message::ChangeMode(Mode::Edit);
                    }
                }
                Char('t') => {
                    if current_mode == Mode::SelectionDeck {
                        return Message::ChangeMode(Mode::Testing);
                    }
                }
                Char('q') | Esc => return Message::ChangeMode(Mode::Quit),
                Char('h') | Left => self.mv_left(),
                Char('l') | Right => self.mv_right(),
                Char('k') | Up => self.mv_up(),
                Char('j') | Down => self.mv_down(),
                Char('d') | Delete => {
                    if self.number_of_elements != 0 {
                        self.current_popup = Some(Popup::new(
                            SelectionPopupTypes::DeleteItem,
                            String::from_str("Do you really want to delete this item").unwrap(),
                            vec![
                                String::from_str("YES").unwrap(),
                                String::from_str("NO").unwrap(),
                            ],
                        ));
                    }
                }
                _ => {}
            };
        } else {
            // If there is a popup
            match self.current_popup.as_ref().unwrap().popup_type {
                // We check what is the purpose of the popup
                SelectionPopupTypes::DeleteItem => {
                    let result = self.current_popup.as_mut().unwrap().handle_key_press(key);
                    if result == self.current_popup.as_mut().unwrap().number_of_buttons {
                    } else {
                        if result == 0 {
                            return Message::Delete(self.cursor_position);
                        }
                        self.current_popup = None;
                    }
                }
            }
        }
        Message::Nothing
    }
}

impl Widget for &SelectionApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render Selection app
        let [viewport, scrollbar_area] =
            Layout::horizontal(vec![Constraint::Min(0), Constraint::Length(1)]).areas(area); // To simplify scrollbar implementation

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("^"))
            .end_symbol(Some("v"));

        let scrollbar_position = ((((self.cursor_position + 1) / self.n_columns)
            + min_by(
                (self.cursor_position + 1) % self.n_columns,
                1,
                |x: &u32, y: &u32| x.cmp(&y),
            ))
        .checked_sub(self.n_lines)
        .or(Some(0))
        .unwrap()) as usize;

        let mut scrollbar_state = ScrollbarState::new(
            (((self.number_of_elements / self.n_columns)
                + min_by(
                    self.number_of_elements % self.n_columns,
                    1,
                    |x: &u32, y: &u32| x.cmp(&y),
                ))
            .checked_sub(self.n_lines)
            .or(Some(0))
            .unwrap()
                + 1) as usize,
        )
        .position(scrollbar_position);

        scrollbar.render(scrollbar_area, buf, &mut scrollbar_state);

        let vertical_layout =
            Layout::vertical(vec![
                Constraint::Percentage((100 / self.n_lines) as u16);
                self.n_lines as usize
            ])
            .split(viewport);

        for i in 0..self.n_lines {
            let horizontal_layout =
                Layout::horizontal(vec![
                    Constraint::Percentage((100 / self.n_columns) as u16);
                    self.n_lines as usize
                ])
                .split(vertical_layout[i as usize]);
            for j in 0..self.n_columns {
                if self.number_of_elements
                    < scrollbar_position as u32 * self.n_columns + i * self.n_lines + j + 1
                {
                    break;
                }
                let element;
                if self.cursor_position
                    == scrollbar_position as u32 * self.n_columns + i * self.n_lines + j
                {
                    element = Paragraph::new(
                        self.elements[scrollbar_position * self.n_columns as usize
                            + (i * self.n_columns + j) as usize]
                            .as_str(),
                    )
                    .block(Block::bordered().border_style(Color::Red));
                } else {
                    element = Paragraph::new(
                        self.elements[scrollbar_position * self.n_columns as usize
                            + (i * self.n_columns + j) as usize]
                            .as_str(),
                    )
                    .block(Block::bordered());
                }
                element.render(horizontal_layout[j as usize], buf);
            }
        }

        // Render eventual popup over this
        if self.current_popup.is_some() {
            self.current_popup.as_ref().unwrap().render(
                Popup::<SelectionPopupTypes>::make_centered_rectangle_area(30, 30, viewport),
                buf,
            );
        }
    }
}
