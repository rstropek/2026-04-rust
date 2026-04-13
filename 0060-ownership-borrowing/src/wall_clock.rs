#[derive(Debug, Clone)]
pub struct WallClock {
    pub hours: u8,
    pub minutes: u8,
}

impl WallClock {
    pub fn new(hours: u8, minutes: u8) -> Self {
        Self { hours, minutes }
    }

    pub fn get_hours(&self) -> u8 {
        self.hours
    }

    pub fn get_minutes(&self) -> u8 {
        self.minutes
    }

    pub fn add_minutes(&mut self, minutes: u8) {
        let total_minutes = self.minutes + minutes;
        self.hours = (self.hours + total_minutes / 60) % 24;
        self.minutes = total_minutes % 60;
    }
}
