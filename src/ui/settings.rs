use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders},
    Frame,
};
use strum::IntoEnumIterator;

use crate::{app::App, enums::settings::Settings};

const COLS: usize = 2;
const ROWS: usize = 3;

pub struct SettingsUI;

impl SettingsUI {
    pub fn render(pomodoro: &mut App, frame: &mut Frame, chunks: Rect) {
        let screen_block = Block::default()
            .title("Settings")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default());

        let inner_area = screen_block.inner(chunks);
        frame.render_widget(screen_block, chunks);

        let layout = Self::create_layout(inner_area);

        for (i, settings) in Settings::iter().enumerate() {
            Self::render_settings(
                pomodoro,
                frame,
                layout[i],
                &settings,
                pomodoro.current_settings == settings,
            );
        }
    }

    fn create_layout(area: Rect) -> Vec<Rect> {
        let cols = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1)].repeat(COLS))
            .split(area);

        let mut cells: Vec<Rect> = Vec::with_capacity(COLS * ROWS);

        for col in cols.iter() {
            let rows = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Fill(1)].repeat(ROWS))
                .split(*col);
            cells.extend(rows.to_vec());
        }
        cells
    }

    fn render_settings(
        pomodoro: &mut App,
        frame: &mut Frame,
        area: Rect,
        settings: &Settings,
        is_selected: bool,
    ) {
        let style = if is_selected {
            Style::default().red()
        } else {
            Style::default()
        };

        let settings_block = Block::default()
            .title(settings.to_string())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .style(style);

        Self::render_menu(pomodoro, settings_block.inner(area), settings);

        frame.render_widget(settings_block, area);
    }

    fn render_menu(pomodoro: &mut App, area: Rect, settings: &Settings) {
        match settings {
            Settings::PomodoroSeconds => Self::render_time_menu(area, pomodoro.pomodoro_seconds),
            Settings::ShortBreakSeconds => {
                Self::render_time_menu(area, pomodoro.short_break_seconds)
            }
            Settings::LongBreakSeconds => Self::render_time_menu(area, pomodoro.long_break_seconds),
            Settings::ShortBreaksBeforeLong => {
                Self::render_count_menu(area, pomodoro.short_breaks_before_long)
            }
            Settings::PomodoroSound => {}
            Settings::Autostart => {}
        }
    }

    fn render_time_menu(area: Rect, seconds: usize) {}

    fn render_count_menu(area: Rect, count: usize) {}
}
