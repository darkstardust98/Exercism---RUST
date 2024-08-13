pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let normalized_hours = ((total_minutes / 60) % 24 + 24) % 24;
        let normalized_minutes = ((total_minutes % 60) + 60) % 60;
        Clock {
            hours: normalized_hours,
            minutes: normalized_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.hours * 60 + self.minutes + minutes;
        Clock::new(0, total_minutes)
    }
}