use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    symbols::border,
    text::Text,
    widgets::{
        block::*, Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    Frame,
};
use std::{cmp, rc::Rc};

use crate::app::{App, AppState};

pub fn ui(f: &mut Frame, app: &mut App) {
    //Matches apps current state and displays accordingly
    match app.state {
        AppState::SelectDeck => {
            //Define the 2 big blocks (content and title)
            let vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
                .split(f.size());

            //Update scrollbar position
            if app.selected_deck_index < app.scrollbar_position * 3 {
                app.scrollbar_position -= 1;
            }

            if app.selected_deck_index
                > app.scrollbar_position * 3 + vertical_layout[1].height as usize - 1
            {
                app.scrollbar_position += 1;
            }

            //Create the scrollbar
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);

            let mut scrollbar_state = ScrollbarState::new(
                (cmp::max(
                    (app.number_of_decks as f32 / 3.0).ceil() as i32
                        - (vertical_layout[1].height / 3) as i32
                        + 1,
                    1,
                )) as usize,
            )
            .position(app.scrollbar_position)
            .viewport_content_length((vertical_layout[1].height / 3 + 1) as usize);

            let mut constraints = Vec::new();

            for _i in 0..cmp::min(
                (app.number_of_decks as f32 / 3.3).ceil() as u16,
                (vertical_layout[1].height as f32 / 3.3).ceil() as u16,
            ) as usize
            {
                constraints.push(Constraint::Length(3))
            }

            let sub_vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(vertical_layout[1]);

            let horizontal_layout: Vec<Rc<[Rect]>> = (0..sub_vertical_layout.len())
                .map(|i| {
                    Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(vec![
                            Constraint::Percentage(100 / 3),
                            Constraint::Percentage(100 / 3),
                            Constraint::Percentage(100 / 3),
                        ])
                        .split(sub_vertical_layout[i as usize])
                })
                .collect();

            let title_block = Block::default()
                .borders(Borders::ALL)
                .border_set(border::THICK)
                .style(Style::default());

            let title = Paragraph::new(Text::styled(
                "Your FlashCards",
                Style::default().fg(Color::Red),
            ))
            .alignment(Alignment::Center)
            .block(title_block);

            //The rendering block
            for j in 0..cmp::min(
                sub_vertical_layout.len(),
                (vertical_layout[1].height as f32 / 3.0).ceil() as usize,
            ) {
                for i in 0..3 {
                    if app.scrollbar_position * 3 + j * 3 + i < app.number_of_decks {
                        let preview_title = Title::from("TODO");

                        let preview_block = Block::default()
                            .borders(Borders::ALL)
                            .border_set(border::ROUNDED)
                            .title(preview_title.alignment(Alignment::Center));

                        if app.scrollbar_position * 3 + j * 3 + i
                            == app.selected_deck_index as usize
                        {
                            let preview_block = preview_block.border_style(Style::new().red());

                            let preview_contents = Paragraph::new(Text::styled(
                                format!("Nbr : {}", app.scrollbar_position * 3 + j * 3 + i),
                                Style::default(),
                            ))
                            .alignment(Alignment::Center)
                            .block(preview_block);
                            f.render_widget(preview_contents, horizontal_layout[j][i]);
                        } else {
                            let preview_contents = Paragraph::new(Text::styled(
                                format!("Nbr : {}", app.scrollbar_position * 3 + j * 3 + i),
                                Style::default(),
                            ))
                            .alignment(Alignment::Center)
                            .block(preview_block);
                            f.render_widget(preview_contents, horizontal_layout[j][i]);
                        }
                    }
                }
            }

            f.render_widget(title, vertical_layout[0]);
            f.render_stateful_widget(
                scrollbar,
                vertical_layout[1].inner(&Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &mut scrollbar_state,
            )
        }
        AppState::SelectCard => {
            let vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
                .split(f.size());

            //Update scrollbar position
            if app.selected_card_index < app.scrollbar_position * 3 {
                app.scrollbar_position -= 1;
            }

            if app.selected_card_index
                > app.scrollbar_position * 3 + vertical_layout[1].height as usize - 1
            {
                app.scrollbar_position += 1;
            }

            //Create the scrollbar
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);

            let mut scrollbar_state = ScrollbarState::new(
                (cmp::max(
                    (app.number_of_decks as f32 / 3.0).ceil() as i32
                        - (vertical_layout[1].height / 3) as i32
                        + 1,
                    1,
                )) as usize,
            )
            .position(app.scrollbar_position)
            .viewport_content_length((vertical_layout[1].height / 3 + 1) as usize);

            let mut constraints = Vec::new();

            for _i in 0..cmp::min(
                (app.number_of_decks as f32 / 3.0).ceil() as u16,
                (vertical_layout[1].height as f32 / 6.0).ceil() as u16,
            ) as usize
            {
                constraints.push(Constraint::Length(6))
            }

            let sub_vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(vertical_layout[1]);

            let horizontal_layout: Vec<Rc<[Rect]>> = (0..sub_vertical_layout.len())
                .map(|i| {
                    Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(vec![
                            Constraint::Percentage(100 / 3),
                            Constraint::Percentage(100 / 3),
                            Constraint::Percentage(100 / 3),
                        ])
                        .split(sub_vertical_layout[i as usize])
                })
                .collect();

            let title_block = Block::default()
                .borders(Borders::ALL)
                .border_set(border::THICK)
                .style(Style::default());

            let title = Paragraph::new(Text::styled(
                "Your Cards in this deck",
                Style::default().fg(Color::Red),
            ))
            .alignment(Alignment::Center)
            .block(title_block);

            //The rendering block
            for j in 0..cmp::min(
                sub_vertical_layout.len(),
                (vertical_layout[1].height as f32 / 6.0).ceil() as usize,
            ) {
                for i in 0..3 {
                    if app.scrollbar_position * 6 + j * 6 + i < app.number_of_decks {
                        let preview_title = Title::from("TODO");

                        let preview_block = Block::default()
                            .borders(Borders::ALL)
                            .border_set(border::ROUNDED)
                            .title(preview_title.alignment(Alignment::Center));

                        if app.scrollbar_position * 3 + j * 3 + i
                            == app.selected_card_index as usize
                        {
                            let preview_block = preview_block.border_style(Style::new().red());

                            let preview_contents = Paragraph::new(Text::styled(
                                format!("Nbr : {}", app.scrollbar_position * 3 + j * 3 + i),
                                Style::default(),
                            ))
                            .alignment(Alignment::Center)
                            .block(preview_block);
                            f.render_widget(preview_contents, horizontal_layout[j][i]);
                        } else {
                            let preview_contents = Paragraph::new(Text::styled(
                                format!("Nbr : {}", app.scrollbar_position * 3 + j * 3 + i),
                                Style::default(),
                            ))
                            .alignment(Alignment::Center)
                            .block(preview_block);
                            f.render_widget(preview_contents, horizontal_layout[j][i]);
                        }
                    }
                }
            }

            f.render_widget(title, vertical_layout[0]);
            f.render_stateful_widget(
                scrollbar,
                vertical_layout[1].inner(&Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &mut scrollbar_state,
            )
        }

        AppState::NewConfig => {
            let popup_title = Title::from("Where to store flashcards ?");
            input_area(69, 7, f, popup_title, app);
        }

        AppState::NewDeck => {
            let popup_title = Title::from("What is the name of the new deck ?");
            input_area(69, 7, f, popup_title, app);
        }
        _ => todo!(),
    }
}

/*
Creates an input area of the designated size
*/
fn input_area(percent_x: u16, percent_y: u16, f: &mut Frame, title: Title, app: &mut App) {
    let span = Span::raw("> ");
    let content_span = Span::raw(&app.input_buffer);
    let popup_area = centered_rect(percent_x, percent_y, f.size());
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title(title.alignment(Alignment::Center));
    let popup_contents =
        Paragraph::new(Text::from(Line::from(vec![span, content_span]))).block(popup_block);
    f.render_widget(popup_contents, popup_area);
    //Cursor positioning
    app.input_buffer_max_size = (popup_area.width - 5) as usize;
    f.set_cursor(
        popup_area.x + 3 + app.cursor_position as u16,
        popup_area.y + 1,
    )
}

/*
Takes 2 integers defining the proportional width and height
compared to the terminal height.

*/
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
