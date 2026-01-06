use strum::{Display, EnumCount, EnumIter};

#[derive(Display, EnumIter, EnumCount)]
pub enum Settings {
    #[strum(to_string = "Pomodoro Time")]
    PomodoroSeconds,
    #[strum(to_string = "Short Break Time")]
    ShortBreakSeconds,
    #[strum(to_string = "Long Break Time")]
    LongBreakSeconds,
    #[strum(to_string = "Short Before Long")]
    ShortBreaksBeforeLong,
}
