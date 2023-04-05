use core::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clock {
    hours:  i32,
    minutes: i32,
}

impl Clock {
   pub fn new(hours: i32, minutes: i32) -> Self {
        let mut m = (hours * 60 + minutes) % (24 * 60);

        if m < 0 {
            m = 24 * 60 + m;
        }

        let h = m / 60;
        m = m % 60;
                
        Self{
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

