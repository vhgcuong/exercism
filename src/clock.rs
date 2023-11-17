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

        time.adjust_minutes();
        time.adjust_hours();

        time
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    fn adjust_hours(&mut self) {
        match self.hours {
            i32::MIN..=-1 => {
                if self.hours % 24 == 0 {
                    self.hours = 0;
                } else {
                    self.hours = self.hours % 24 + 24;
                }
            },
            24..=i32::MAX => {
                self.hours = self.hours % 24;
            },
            _ => {}
        }
    }

    fn adjust_minutes(&mut self) {
        match self.minutes {
            i32::MIN..=-1 => {
                if self.minutes % 60 == 0 {
                    self.hours += self.minutes / 60;
                    self.minutes = 0;
                } else {
                    self.hours += self.minutes / 60 - 1;
                    self.minutes = self.minutes % 60 + 60;
                }
            },
            60..=i32::MAX => {
                self.hours += self.minutes / 60;
                self.minutes = self.minutes % 60;
            },
            _ => {}
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
