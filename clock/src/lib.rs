use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = hours % 24 + 24;
        let hours = (hours + minutes / 60) % 24;
        let minutes = minutes % 60;
        if minutes < 0 {
            Clock {
                hours: (hours - (minutes / 60 + 1) + 24) % 24,
                minutes: minutes + 60,
            }
        } else {
            Clock { hours, minutes }
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mins = (minutes + self.minutes) % 60;
        let hours = (self.hours + ((self.minutes + minutes) / 60)) % 24;
        if mins < 0 {
            Clock {
                hours: (hours - (mins / 60 + 1) + 24) % 24,
                minutes: mins + 60,
            }
        } else {
            Clock {
                hours,
                minutes: mins,
            }
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
