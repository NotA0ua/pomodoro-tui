use ratatui::{
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{app::App, enums::pomodoros::Pomodoros};

pub fn render_pomodoro(pomodoro: &App, frame: &mut Frame, chunks: Rect) {
    let screen_style = match pomodoro.current_type {
        Pomodoros::Pomodoro => Style::default().red(),
        Pomodoros::ShortBreak => Style::default().cyan(),
        Pomodoros::LongBreak => Style::default().yellow(),
    };

    let screen_block = Block::default()
        .title(match pomodoro.current_type {
            Pomodoros::Pomodoro => "Pomodoro",
            Pomodoros::ShortBreak => "Short Break",
            Pomodoros::LongBreak => "Long Break",
        })
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(screen_style);

    let padding =
        (chunks.height as usize - 5 - if pomodoro.is_pomodoro_running { 0 } else { 1 }) / 2;

    let pomodoro_text = Text::styled(
        format!(
            "{}{}\n{:02}:{:02}\n{}\n{}",
            "\n".repeat(padding),
            "•".repeat((pomodoro.elapsed_seconds % 10) * 2 + 1),
            pomodoro.elapsed_seconds / 60,
            pomodoro.elapsed_seconds % 60,
            "•".repeat((pomodoro.elapsed_seconds % 10) * 2 + 1),
            {
                if pomodoro.is_pomodoro_running == false {
                    "⏸"
                } else {
                    ""
                }
            }
        ),
        Style::default().white(),
    );

    let pomodoro_paragraph = Paragraph::new(pomodoro_text)
        .style(Style::default())
        .centered()
        .block(screen_block);

    frame.render_widget(pomodoro_paragraph, chunks);
}
