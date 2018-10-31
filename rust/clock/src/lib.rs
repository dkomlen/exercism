use std::fmt;

#[derive(Debug)]
pub struct Clock {
    pub minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { minutes: 60 * hours + minutes }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock { minutes: self.minutes + minutes }
    }

    pub fn get_minutes(&self) -> i32 {
        match self.minutes % (24 * 60) {
            x if x < 0 => x + 24 * 60,
            x => x
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mins = self.get_minutes();
        let hours = (mins / 60) % 24;
        let minutes = mins % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.get_minutes().eq(&other.get_minutes())
    }
}