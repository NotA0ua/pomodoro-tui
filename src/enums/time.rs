pub enum Time {
    Seconds(usize),
    Minutes(usize),
    Hours(usize),
}

impl Time {
    pub fn as_seconds(&self) -> usize {
        match *self {
            Time::Seconds(seconds) => seconds,
            Time::Minutes(minutes) => minutes * 60,
            Time::Hours(hours) => hours * 3600,
        }
    }
    
}
