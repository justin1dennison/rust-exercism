use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const SECONDS_PER_DAY: i32 = 86400;
const SECONDS_PER_HOUR: i32 = 3600;
const SECONDS_PER_MINUTE: i32 = 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::from_seconds(SECONDS_PER_HOUR * hours +  SECONDS_PER_MINUTE * minutes)
    }

    fn from_seconds(seconds: i32) -> Self {
       let seconds = (seconds % SECONDS_PER_DAY + SECONDS_PER_DAY) % SECONDS_PER_DAY;
       let hours = seconds / SECONDS_PER_HOUR;
       let minutes = (seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
       Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
