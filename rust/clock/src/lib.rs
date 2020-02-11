use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Clock {{ x: {}, y: {} }}", self.hours, self.minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hours = self.hours.to_string();
        let mut minutes = self.minutes.to_string();
        if self.hours < 10 {
            hours = "0".to_owned() + &hours;
        }
        if self.minutes < 10 {
            minutes = "0".to_owned() + &minutes;
        }
        write!(f, "{}:{}", hours, minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        let mut hours = hours % 24;
        let mut minutes = minutes;

        if minutes % 60 != 0 || minutes > 60 - 1 {
            let rest = minutes % 60;
            hours = (hours + (minutes - rest) / 60) % 24;
            minutes = rest;
        }

        if minutes < 0 {
            hours = hours - 1;
            minutes = 60 - minutes.abs();
        }

        if hours < 0 {
            hours = 24 - hours.abs();
        }

        Clock {
            hours: hours % 24,
            minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, minutes + self.minutes)
    }
}
