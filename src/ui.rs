use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    symbols::border,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, block::*},
    Frame,
};
use std::rc::Rc;

use crate::app::{App, AppState};

pub fn ui(f: &mut Frame, app: &mut App, needed_previews : i8) {
    match app.state {
        AppState::MainScreen => { //TEMPORARY
            let outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                             Constraint::Length(3),
                             Constraint::Min(7),
                             Constraint::Min(7),
                             Constraint::Min(7),
                ])
                .split(f.size());

            let lvls_layout : Vec<Rc<[Rect]>> = (1..4).map(|i| Layout::default()
                                                    .direction(Direction::Horizontal)
                                                    .constraints(vec![
                                                                 Constraint::Min(12),
                                                                 Constraint::Min(12),
                                                                 Constraint::Min(12),
                                                    ])
                                                    .split(outer_layout[i])).collect();

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

            let mut previews_number = 0;

            for lvl in lvls_layout {
                for i in 0..3 {
                    previews_number += 1;

                    if previews_number <= needed_previews {
                        let preview_title = Title::from("TODO");

                        let preview_block = Block::default()
                            .borders(Borders::ALL)
                            .border_set(border::ROUNDED)
                            .title(preview_title.alignment(Alignment::Center));

                        let preview_contents = Paragraph::new(Text::styled("TODOTO", Style::default()))
                            .alignment(Alignment::Center)
                            .block(preview_block);
                        f.render_widget(preview_contents, lvl[i]);
                    }
                }
            }

            f.render_widget(title, outer_layout[0]);},

        AppState::NewConfig => {

            let span = Span::raw("> ");
            let content_span = Span::raw(&app.input_buffer);
            let popup_title = Title::from("Where to store flashcards");
            let popup_area = centered_rect(69, 6, f.size());
            let popup_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title(popup_title.alignment(Alignment::Center));
            let popup_contents = Paragraph::new(Text::from(Line::from(vec![span, content_span])))
                .block(popup_block);
            f.render_widget(popup_contents, popup_area);
            //Cursor positioning
            app.input_buffer_max_size = popup_area.width - 5;
            f.set_cursor(popup_area.x + 3 + app.cursor_position, popup_area.y + 1);

        }
        _ => todo!(),
    }
}

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
