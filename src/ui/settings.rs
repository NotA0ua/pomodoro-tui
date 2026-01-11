use ratatui::{
    crossterm::event::{KeyCode, KeyModifiers},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders},
    Frame,
};
use strum::{EnumCount, IntoEnumIterator};

use crate::{app::App, enums::settings::Setting};

const COLS: usize = 2;
const ROWS: usize = 3;

pub struct Settings;

impl Settings {
    pub fn fetch_keys(pomodoro: &mut App, key: KeyCode, modifier: KeyModifiers) {
        match modifier {
            KeyModifiers::CONTROL => match key {
                KeyCode::Left => {
                    pomodoro.current_setting = Setting::iter()
                        .nth(if pomodoro.current_setting as usize == 0 {
                            Setting::COUNT - 1
                        } else {
                            pomodoro.current_setting as usize - 1
                        })
                        .unwrap();
                }
                KeyCode::Right => {
                    pomodoro.current_setting = Setting::iter()
                        .nth(
                            if (pomodoro.current_setting as usize) < (Setting::COUNT - 1) {
                                pomodoro.current_setting as usize + 1
                            } else {
                                0
                            },
                        )
                        .unwrap();
                }
                _ => {}
            },
            _ => match key {
                KeyCode::Left => match pomodoro.current_setting {
                    Setting::PomodoroTime | Setting::ShortBreakTime | Setting::LongBreakTime => {
                        if pomodoro.current_setting_index == 0 {
                            pomodoro.current_setting_index = 2;
                        } else {
                            pomodoro.current_setting_index -= 1;
                        }
                    }
                    _ => {}
                },
                KeyCode::Right => match pomodoro.current_setting {
                    Setting::PomodoroTime | Setting::ShortBreakTime | Setting::LongBreakTime => {
                        if pomodoro.current_setting_index == 2 {
                            pomodoro.current_setting_index = 0;
                        } else {
                            pomodoro.current_setting_index += 1;
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
        };
    }

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

        for (i, settings) in Setting::iter().enumerate() {
            Self::render_settings(
                pomodoro,
                frame,
                layout[i],
                &settings,
                pomodoro.current_setting == settings,
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
        settings: &Setting,
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

    fn render_menu(pomodoro: &mut App, area: Rect, settings: &Setting) {
        match settings {
            Setting::PomodoroTime => Self::render_time_menu(pomodoro, area),
            Setting::ShortBreakTime => Self::render_time_menu(pomodoro, area),
            Setting::LongBreakTime => Self::render_time_menu(pomodoro, area),
            Setting::ShortBreaksBeforeLong => Self::render_count_menu(pomodoro, area),
            Setting::PomodoroSound => Self::render_dropdown_menu(pomodoro, area),
            Setting::Autostart => Self::render_toggle_menu(pomodoro, area),
        }
    }

    fn render_time_menu(pomodoro: &mut App, area: Rect) {}

    fn render_count_menu(pomodoro: &mut App, area: Rect) {}

    fn render_toggle_menu(pomodoro: &mut App, area: Rect) {}

    fn render_dropdown_menu(pomodoro: &mut App, area: Rect) {}
}
