use ratatui::{
    Frame, layout::{Alignment, Rect}, style::Style, text::Text, widgets::{Block, BorderType, Borders, Paragraph}
};

use crate::{app::App, enums::pomodoros::Pomodoros};

pub fn render_pomodoro(pomodoro: &App, frame: &mut Frame, chunks: Rect) {
    let screen_block = Block::default()
        .title(match pomodoro.current_type {
            Pomodoros::Pomodoro => "Pomodoro",
            Pomodoros::ShortBreak => "Short break",
            Pomodoros::LongBreak => "Long break",
        })
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default());

    let pomodoro_text = Text::styled(
        format!(
            "Pomdoros: {}\nShort breaks: {}\nLong breaks: {}\nElapsed time: {}m {}s\n{}\n{}",
            pomodoro.pomdoros,
            pomodoro.short_breaks,
            pomodoro.long_breaks,
            pomodoro.elapsed_seconds / 60,
            pomodoro.elapsed_seconds % 60,
            "•".repeat((pomodoro.elapsed_seconds % 10) * 2 + 1),
            {
                if pomodoro.is_pomodoro_running == false {
                    "Paused"
                } else {
                    ""
                }
            }
        ),
        Style::default(),
    );

    let pomodoro_paragraph = Paragraph::new(pomodoro_text)
        .style(Style::default())
        .centered()
        .block(screen_block);

    frame.render_widget(pomodoro_paragraph, chunks);
}
