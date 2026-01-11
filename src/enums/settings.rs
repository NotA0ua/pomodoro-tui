use strum::{Display, EnumCount, EnumIter};

#[derive(Display, EnumIter, EnumCount, PartialEq, Eq, Clone, Copy)]
pub enum Setting {
    #[strum(to_string = "Pomodoro Time")]
    PomodoroTime,
    #[strum(to_string = "Short Break Time")]
    ShortBreakTime,
    #[strum(to_string = "Long Break Time")]
    LongBreakTime,
    #[strum(to_string = "Short Before Long")]
    ShortBreaksBeforeLong,
    #[strum(to_string = "Pomodoro Sound")]
    PomodoroSound,
    #[strum(to_string = "Autostart")]
    Autostart,
}
