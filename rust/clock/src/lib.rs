use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let m = minutes.rem_euclid(60);
        let h = (hours + minutes.div_euclid(60)).rem_euclid(24);
        Clock {
            hours: h as u8,
            minutes: m as u8,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours as i32, self.minutes as i32 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
