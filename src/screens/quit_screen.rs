use ratatui::{
    Frame, style::{Color, Style}, text::{Line, Text}, widgets::{Block, Borders, Paragraph}
};

use crate::ui::centered_rect;

pub fn render_quit (frame: &mut Frame) {
    let screen_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let quit_text = Line::styled("Do you really want to quit?", Style::default());
    let quit_keys_text = Line::styled("(q/Esc)", Style::default().fg(Color::Red));

    let quit_paragraph = Paragraph::new(Text::from(vec![quit_text, quit_keys_text]))
        .style(Style::default())
        .centered()
        .block(screen_block);

    let area = centered_rect(60, 25, frame.area());

    frame.render_widget(quit_paragraph, area);
}
