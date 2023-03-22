use std::cmp::Ordering;
use crate::utils::DEFAULT_FPS;

#[derive(Eq)]
pub struct TimeStamp {
    minute: u8,
    second: u8,
    frame: u8
}

impl TimeStamp {
    pub fn increment(&mut self) {
        self.frame += 1;

        if self.frame >= DEFAULT_FPS {
            self.frame = 0;
            self.second += 1;
        }

        if self.second > 59 {
            self.second = 0;
            self.minute = 1;
        }

    }

    pub fn time_as_tuple(&self) -> (u8, u8, u8) {
        (self.minute, self.second, self.frame)
    }

    pub fn time_as_int(&self) -> u32 {
        (self.minute * 60 * DEFAULT_FPS + self.second * DEFAULT_FPS + self.frame).into()
    }
}

impl PartialEq<Self> for TimeStamp {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd for TimeStamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}