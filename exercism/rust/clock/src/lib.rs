#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours_from_minutes = minutes.div_euclid(60);

        Self {
            hours: (hours + hours_from_minutes).rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
