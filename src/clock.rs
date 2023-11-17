use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut time: Clock = Clock {
            hours,
            minutes
        };

        match minutes {
            i32::MIN..=-1 => {
                if minutes % 60 == 0 {
                    time.minutes = 0;
                    time.hours += minutes / 60;
                } else {
                    time.minutes = minutes % 60 + 60;
                    time.hours += minutes / 60 - 1;
                }
            },
            60..=i32::MAX => {
                time.minutes = minutes % 60;
                time.hours += minutes / 60;
            },
            _ => {}
        }

        match time.hours {
            i32::MIN..=-1 => {
                if time.hours % 24 == 0 {
                    time.hours = 0;
                } else {
                    time.hours = time.hours % 24 + 24;
                }
            },
            24..=i32::MAX => {
                time.hours = time.hours % 24;
            },
            _ => {}
        }

        time
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
