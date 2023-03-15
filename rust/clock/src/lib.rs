const MINS_DAY: i32 = 24 * 60;
const MINS_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    mins: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            mins: (((hours * MINS_HOUR + minutes) % MINS_DAY) + MINS_DAY) % MINS_DAY,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            mins: (((self.mins + minutes) % MINS_DAY) + MINS_DAY) % MINS_DAY,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.mins / MINS_HOUR % 24, self.mins % 60)
    }
}
