use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::string::ToString;

pub struct Clock {
    hours: u32,
    minutes: u32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut m = minutes;
        let mut mc = 0;
        let mut fc = 0;
        while m < 0 {
            m += 60;
            mc += 1;
        }

        while m > 59 {
            m -= 60;
            fc += 1;
        }

        let mut h = hours - mc + fc;
        while h < 0 {
            h += 24;
        }
        while h > 23 {
            h -= 24;
        }

        Clock {
            hours: h as u32,
            minutes: m as u32,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut m = self.minutes as i32 + minutes;
        let mut mc = 0;
        while m < 0 {
            m += 60;
            mc += 1;
        }

        let mut fc = 0;
        while m > 59 {
            m -= 60;
            fc += 1;
        }

        let mut h = self.hours as i32 - mc + fc;
        while h < 0 {
            h += 24;
        }
        while h > 23 {
            h -= 24;
        }

        Clock {
            hours: h as u32,
            minutes: m as u32,
        }
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Eq for Clock {}

impl Debug for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Clock {{ hours: {}, minutes: {} }}",
            self.hours, self.minutes
        )
    }
}
