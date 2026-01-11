use std::{
    error::Error,
    time::{Duration, Instant},
};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    prelude::Backend,
    Frame, Terminal,
};

use crate::{
    enums::{pomodoros::Pomodoros, screens::Screens, settings::Setting, time::Time},
    ui::{
        info::render_info, pomodoro::render_pomodoro, quit::render_quit, settings::Settings, tabs::render_tabs
    },
    utils::sound::play_timer_sound,
};

pub struct App {
    pub is_running: bool,
    pub is_pomodoro_running: bool,

    pub current_screen: Screens,
    pub last_screen: Screens,

    pub current_type: Pomodoros,
    pub current_setting: Setting,
    pub current_setting_index: usize,

    pub pomodoro_seconds: usize,
    pub short_break_seconds: usize,
    pub long_break_seconds: usize,

    pub short_breaks_before_long: usize,

    pub pomdoros: usize,
    pub short_breaks: usize,
    pub long_breaks: usize,

    pub elapsed_seconds: usize,
}

impl Default for App {
    fn default() -> Self {
        Self::new(Time::Minutes(25), Time::Minutes(5), Time::Minutes(15), 2)
    }
}

impl App {
    pub fn new(
        pomodoro_time: Time,
        short_break_time: Time,
        long_break_time: Time,
        short_breaks_before_long: usize,
    ) -> Self {
        App {
            is_running: true,
            is_pomodoro_running: false,
            current_screen: Screens::Pomodoro,
            last_screen: Screens::Pomodoro,
            current_type: Pomodoros::Pomodoro,
            current_setting: Setting::PomodoroTime,
            current_setting_index: 0,
            pomodoro_seconds: pomodoro_time.as_seconds(),
            short_break_seconds: short_break_time.as_seconds(),
            long_break_seconds: long_break_time.as_seconds(),
            short_breaks_before_long,
            pomdoros: 0,
            short_breaks: 0,
            long_breaks: 0,
            elapsed_seconds: 0,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        let tick_rate = Duration::from_millis(1000);
        let mut last_tick = Instant::now();
        while self.is_running {
            terminal.draw(|f| self.draw_ui(f))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                self.check_keys()?;
            }
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn check_keys(&mut self) -> Result<(), Box<dyn Error>> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                return Ok(());
            }
            match key.code {
                KeyCode::Char(' ') => match self.current_screen {
                    Screens::Pomodoro => self.is_pomodoro_running = !self.is_pomodoro_running,
                    _ => {}
                },
                KeyCode::Char('q') => {
                    if let Screens::Quit = self.current_screen {
                        self.is_running = false;
                    }
                    self.last_screen = self.current_screen;
                    self.current_screen = Screens::Quit;
                }
                KeyCode::Esc => {
                    if let Screens::Quit = self.current_screen {
                        self.current_screen = self.last_screen;
                    }
                }
                KeyCode::Backspace => {
                    if let Screens::Pomodoro = self.current_screen {
                        self.elapsed_seconds = 0;
                    }
                }
                KeyCode::Tab => {
                    self.current_screen = self.current_screen.next();
                }
                KeyCode::BackTab => {
                    self.current_screen = self.current_screen.previous();
                },
                _ => {}
            }
            Settings::fetch_keys(self, key.code, key.modifiers);
        }
        Ok(())
    }

    fn draw_ui(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(frame.area());

        render_tabs(self, frame, chunks[0]);
        render_info(self, frame, chunks[2]);

        match self.current_screen {
            Screens::Pomodoro => render_pomodoro(self, frame, chunks[1]),
            Screens::Settings => Settings::render(self, frame, chunks[1]),
            Screens::Quit => render_quit(frame),
            Screens::Stats => {}
        }
    }

    fn on_tick(&mut self) {
        if self.is_pomodoro_running {
            self.elapsed_seconds += 1;
        }
        match self.current_type {
            Pomodoros::Pomodoro => {
                if self.elapsed_seconds >= self.pomodoro_seconds {
                    self.elapsed_seconds = 0;
                    self.pomdoros += 1;
                    play_timer_sound();

                    if ((self.short_breaks % (self.short_breaks_before_long)) == 0)
                        && (self.short_breaks != 0)
                    {
                        self.current_type = Pomodoros::LongBreak;
                        return;
                    }
                    self.current_type = Pomodoros::ShortBreak;
                }
            }
            Pomodoros::ShortBreak => {
                if self.elapsed_seconds >= self.short_break_seconds {
                    self.elapsed_seconds = 0;
                    self.short_breaks += 1;
                    play_timer_sound();

                    self.current_type = Pomodoros::Pomodoro;
                }
            }
            Pomodoros::LongBreak => {
                if self.elapsed_seconds >= self.long_break_seconds {
                    self.elapsed_seconds = 0;
                    self.long_breaks += 1;
                    play_timer_sound();

                    self.current_type = Pomodoros::Pomodoro;
                }
            }
        }
    }
}
