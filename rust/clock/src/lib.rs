#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut newh = hours;
        let mut newm = minutes;

        if newm.abs() >= 60 {
            newh = newh + newm / 60;
            newm = newm % 60;
        }
        if newh.abs() >= 24 {
            newh = newh % 24;
        }
        if newm < 0 {
            newh = newh + newm / 60 - (newm % 60 != 0) as i32;
            newm = 60 + newm;
        }
        if newh < 0 {
            newh = 24 + newh;
        }

        Self {
            hours: newh,
            minutes: newm,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut newh = self.hours;
        let mut newm = self.minutes + minutes;

        if newm.abs() >= 60 {
            newh = newh + newm / 60;
            newm = newm % 60;
        }
        if newh.abs() >= 24 {
            newh = newh % 24;
        }
        if newm < 0 {
            newh = newh + newm / 60 - (newm % 60 != 0) as i32;
            newm = 60 + newm;
        }
        if newh < 0 {
            newh = 24 + newh;
        }

        Self {
            hours: newh,
            minutes: newm,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:02}", self.hours).to_string() + ":" + &format!("{:02}", self.minutes).to_string()
    }
}
