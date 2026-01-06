use strum::{Display, EnumCount, EnumIter, FromRepr};

#[derive(Clone, Copy, Display, EnumIter, EnumCount, FromRepr)]
pub enum Screens {
    Pomodoro,
    Settings,
    Stats,
    Quit,
}

impl Screens {
    pub fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = if current_index == 0 {Self::COUNT - 1} else {current_index - 1};
        Self::from_repr(previous_index).unwrap_or(self)
    }

    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = if current_index == (Self::COUNT - 1) {0} else {current_index + 1};
        Self::from_repr(next_index).unwrap_or(self)
    }
}
