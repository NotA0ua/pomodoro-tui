use std::{
    error::Error,
    time::{Duration, Instant},
};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::Backend,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

use crate::{
    enums::{pomodoros::Pomodoros, screens::Screens},
    sound::play_timer_sound,
    ui::centered_rect,
};

pub struct App {
    is_running: bool,
    is_pomodoro_running: bool,
    current_screen: Screens,
    current_type: Pomodoros,
    pomodoro_time: usize,
    short_break_time: usize,
    long_break_time: usize,
    pomdoros: usize,
    short_breaks: usize,
    long_breaks: usize,
    short_breaks_before_long: usize,
    elapsed_seconds: usize,
}

impl Default for App {
    fn default() -> Self {
        App {
            is_running: true,
            is_pomodoro_running: false,
            current_screen: Screens::Main,
            current_type: Pomodoros::Pomodoro,
            pomodoro_time: 20 * 60,
            short_break_time: 5 * 60,
            long_break_time: 15 * 60,
            pomdoros: 0,
            short_breaks: 0,
            long_breaks: 0,
            short_breaks_before_long: 2,
            elapsed_seconds: 0,
        }
    }
}

impl App {
    pub fn new(
        pomodoro_time: usize,
        short_break_time: usize,
        long_break_time: usize,
        short_breaks_before_long: usize,
    ) -> Self {
        App {
            is_running: true,
            is_pomodoro_running: false,
            current_screen: Screens::Main,
            current_type: Pomodoros::Pomodoro,
            pomodoro_time,
            short_break_time,
            long_break_time,
            pomdoros: 0,
            short_breaks: 0,
            long_breaks: 0,
            short_breaks_before_long,
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
                KeyCode::Char('q') => {
                    match self.current_screen {
                        Screens::Quit => self.is_running = false,
                        Screens::Pomodoro => self.is_pomodoro_running = false,
                        _ => {}
                    }
                    self.current_screen = Screens::Quit;
                }

                KeyCode::Char(' ') => match self.current_screen {
                    Screens::Main => {
                        self.current_screen = Screens::Pomodoro;
                        self.is_pomodoro_running = true;
                    }
                    Screens::Pomodoro => self.is_pomodoro_running = !self.is_pomodoro_running,
                    _ => {}
                },

                KeyCode::Esc => match self.current_screen {
                    Screens::Pomodoro => {
                        self.is_pomodoro_running = false;
                        self.current_screen = Screens::Main;
                    }
                    Screens::Quit => {
                        self.current_screen = Screens::Main;
                    }
                    _ => {}
                },

                _ => {}
            }
        }
        Ok(())
    }

    fn draw_ui(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(frame.area());

        let title_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            "Pomodoro timer",
            Style::default().fg(Color::Yellow),
        ))
        .alignment(Alignment::Center)
        .centered()
        .block(title_block);

        frame.render_widget(title, chunks[0]);

        match self.current_screen {
            Screens::Main => {
                let screen_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default());

                let main_span_1 = Span::styled("Press", Style::default());
                let main_key_span = Span::styled(" Space ", Style::default().fg(Color::Red));
                let main_span_2 = Span::styled("to start round", Style::default());

                let main_text =
                    Text::from(Line::from(vec![main_span_1, main_key_span, main_span_2]));

                let main_paragraph = Paragraph::new(main_text)
                    .style(Style::default())
                    .centered()
                    .block(screen_block);

                frame.render_widget(main_paragraph, chunks[1]);
            }
            Screens::Pomodoro => {
                let screen_block = Block::default()
                    .title(match self.current_type {
                        Pomodoros::Pomodoro => "Pomodoro",
                        Pomodoros::ShortBreak => "Short break",
                        Pomodoros::LongBreak => "Long break",
                    })
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default());

                let pomodoro_text = Text::styled(
                    format!(
                        "Pomdoros: {}\nShort breaks: {}\nLong breaks: {}\nElapsed time: {}m {}s\n{}\n{}",
                        self.pomdoros,
                        self.short_breaks,
                        self.long_breaks,
                        // (self.elapsed_seconds / 60) as usize,
                        self.elapsed_seconds / 60,
                        self.elapsed_seconds%60,
                        "•".repeat(self.elapsed_seconds % 10),
                        {
                            if self.is_pomodoro_running == false {
                                "Paused"
                            }else {
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

                frame.render_widget(pomodoro_paragraph, chunks[1]);
            }
            Screens::Quit => {
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
        }
    }

    fn on_tick(&mut self) {
        if self.is_pomodoro_running {
            self.elapsed_seconds += 1;
        }
        match self.current_type {
            Pomodoros::Pomodoro => {
                if self.elapsed_seconds >= self.pomodoro_time {
                    play_timer_sound();
                    self.pomdoros += 1;
                    self.elapsed_seconds = 0;

                    if self.short_breaks == self.short_breaks_before_long {
                        self.current_type = Pomodoros::LongBreak;
                        return;
                    }
                    self.current_type = Pomodoros::ShortBreak;
                }
            }
            Pomodoros::ShortBreak => {
                if self.elapsed_seconds >= self.short_break_time {
                    self.short_breaks += 1;
                    self.elapsed_seconds = 0;
                    self.current_type = Pomodoros::Pomodoro;
                }
            }
            Pomodoros::LongBreak => {
                if self.elapsed_seconds >= self.long_break_time {
                    self.long_breaks += 1;
                    self.elapsed_seconds = 0;
                    self.current_type = Pomodoros::Pomodoro;
                }
            }
        }
    }
}
