use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::Text,
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

pub fn render_info(pomodoro: &App, frame: &mut Frame, chunks: Rect) {
    let pomodoro_text = Text::styled(
        format!(
            "🍅: {}   ⌛: {}   🕰️: {}",
            pomodoro.pomdoros, pomodoro.short_breaks, pomodoro.long_breaks
        ),
        Style::default().add_modifier(Modifier::BOLD),
    );

    let pomodoro_paragraph = Paragraph::new(pomodoro_text)
        .style(Style::default())
        .centered();

    frame.render_widget(pomodoro_paragraph, chunks);
}
