use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::from_minutes(minutes + 60 * hours)
    }

    fn from_minutes(minutes: i32) -> Self {
        let hs =  minutes / 60 % 24;
        let ms = minutes % 60;
        let (hours, minutes) = if minutes < 0 {
            let hours = 24 + hs - 1;
            let minutes = 60 + ms;
            (hours + minutes / 60, minutes % 60)
        } else {
            let hours = (24 + hs) % 24;
            let minutes = (60 + ms) % 60;
            (hours + minutes / 60, minutes % 60)
        };
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
