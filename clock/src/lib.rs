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
        let (hours, minutes) = if minutes < 0 {
            let hours = 24 + (minutes / 60 % 24) - 1;
            let minutes = 60 + (minutes % 60);
            if minutes != 60 { (hours, minutes) } else { (hours + 1, 00) }
        } else {
            let hours = (minutes / 60  + 24) % 24;
            let minutes = minutes % 60;
            (hours, minutes)
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
