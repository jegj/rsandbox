use std::fmt::{self};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }.adjust()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }
        .adjust()
    }

    pub fn adjust(&self) -> Self {
        let remaing_hours = if self.hours < 0 {
            self.hours % 24 + 24
        } else {
            self.hours % 24
        };

        let additional_hours: i32;
        let mut remaing_minutes: i32;

        if self.minutes < 0 {
            additional_hours = self.minutes / 60 + 24 - 1;
            remaing_minutes = self.minutes % 60 + 60;
        } else {
            additional_hours = self.minutes / 60;
            remaing_minutes = self.minutes % 60;
        }

        let mut hours = (remaing_hours + additional_hours) % 24;
        if hours >= 0 {
            if remaing_minutes == 60 {
                remaing_minutes = 0;
                hours += 1;
            }
        } else {
            hours += 24;
        }

        Clock {
            hours,
            minutes: remaing_minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
