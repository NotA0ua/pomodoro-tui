use ratatui::{
    Frame, layout::{Alignment, Rect}, style::{Style, Stylize}, symbols, widgets::{Block, BorderType, Borders, Tabs}
};

use crate::{app::App};

pub fn render_settings(pomodoro: &App, frame: &mut Frame, chunks: Rect) {
    let screen_block = Block::default()
        .title("Settings")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
        .block(screen_block)
        .style(Style::default().white())
        .highlight_style(Style::default().yellow())
        .select(2)
        .divider(symbols::DOT)
        .padding("->", "<-");

    // let pomodoro_paragraph = Paragraph::new(settings_text)
    //     .style(Style::default())
    //     .centered()
    //     .block(screen_block);

    frame.render_widget(tabs, chunks);
}
