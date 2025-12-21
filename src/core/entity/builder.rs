use crate::core::mutator::timestamp::TimeStamp;

pub struct EntityBuilder {
    active_intervals: Vec<[TimeStamp; 2]>,
}

impl EntityBuilder {
    pub fn new() -> EntityBuilder {
        Self {
            active_intervals: Vec::new(),
        }
    }

    pub fn add_active_interval(&mut self, interval: [TimeStamp; 2]) -> &mut Self {
        self.active_intervals.push(interval);
        self
    }

    pub fn build_active_ranges(self) -> Option<Vec<[TimeStamp; 2]>> {
        if self.active_intervals.is_empty() {
            None
        } else {
            Some(self.active_intervals)
        }
    }
}
