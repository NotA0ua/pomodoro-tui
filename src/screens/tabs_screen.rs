use ratatui::{Frame, layout::{Constraint, Layout, Rect}, style::{Color, Style}, widgets::{Block, BorderType, Borders, Tabs}};
use strum::IntoEnumIterator;

use crate::{app::App, enums::screens::Screens};

pub fn render_tabs(pomodoro: &App, frame: &mut Frame, chunks: Rect) {
    let tab_width: u16 = Screens::iter()
        .map(|s| s.to_string().len() as u16 + 2)
        .sum();

    let padding_left = (frame.area().width - tab_width) / 2;

    let centered_tabs = Layout::horizontal([
        Constraint::Length(padding_left - 1),
        Constraint::Length(tab_width + 2),
    ])
    .split(chunks);

    let tabs_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .style(Style::default());

    let tabs = Tabs::new(Screens::iter().map(|s| s.to_string()))
        .block(tabs_block)
        .style(Style::default())
        .highlight_style(
            Style::default().fg(Color::Red), // .bg(Color::White)
        )
        .select(pomodoro.current_screen.clone() as usize)
        .divider("")
        .padding(" ", " ");

    frame.render_widget(tabs, centered_tabs[1]);
}
