use std::fmt;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Clock {
    minutes: i32,
}

const MAX_MINUTES: i32 = 24 * 60;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes - (hours * 60);

        write!(f, "{:0>2}:{:0>2}", hours, minutes)
    }
}

fn wrap_minutes(minutes: i32) -> i32 {
    let wrapped = minutes % MAX_MINUTES;

    if wrapped < 0 {
        MAX_MINUTES + wrapped
    } else {
        wrapped
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: wrap_minutes(hours * 60 + minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: wrap_minutes(self.minutes + minutes),
        }
    }
}
