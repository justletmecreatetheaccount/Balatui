use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::*,
    symbols::border,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, block::*},
    Frame,
};
use std::rc::Rc;

use crate::app::{App, AppState};

pub fn ui(f: &mut Frame, app: &App, needed_previews : i8) {
    match app.state {
        AppState::MainScreen | AppState::NewConfig => { //TEMPORARY
            let outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                             Constraint::Length(3),
                             Constraint::Min(7),
                             Constraint::Min(7),
                             Constraint::Min(7),
                ])
                .split(f.size());

            let lvls : Vec<Rc<[Rect]>> = (1..4).map(|i| Layout::default()
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

            for lvl in lvls {
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
        _ => todo!(),
    }
}
