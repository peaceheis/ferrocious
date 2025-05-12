use crate::core::utils::defaults::DEFAULT_FPS;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt;
use std::fmt::Formatter;

#[derive(Eq, Debug, Default, Copy, Clone)]
pub struct TimeStamp {
    pub minute: u8,
    pub second: u8,
    pub frame: u8,
}

impl TimeStamp {
    pub fn new(minute: u8, second: u8, frame: u8) -> Self {
        TimeStamp {
            minute,
            second,
            frame,
        }
    }

    pub fn as_num_frames(&self, fps: u8) -> u32 {
        (self.minute * 60 * fps + self.second * fps + self.frame) as u32
    }

    pub fn new_with_defaults(minute: Option<u8>, second: Option<u8>, frame: Option<u8>) -> Self {
        TimeStamp {
            minute: minute.unwrap_or(0),
            second: second.unwrap_or(0),
            frame: frame.unwrap_or(0)
        }
    }



    pub fn increment(&mut self, fps: u8) {
        self.frame += 1;

        if self.frame >= fps {
            self.frame = 0;
            self.second += 1;
        }

        if self.second > 59 {
            self.second = 0;
            self.minute += 1;
        }
    }

    pub fn time_as_array(&self) -> [u8;3] {
        [self.minute, self.second, self.frame]
    }
}

impl PartialEq<Self> for TimeStamp {
    fn eq(&self, other: &Self) -> bool {
        self.minute.eq(&other.minute)
            && self.second.eq(&other.second)
            && self.frame.eq(&other.frame)
    }
}

impl PartialOrd for TimeStamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return {
            if self < other {
                Some(Less)
            } else if self == other {
                Some(Equal)
            } else {
                Some(Greater)
            }
        };
    }

    fn lt(&self, other: &Self) -> bool {
        other.minute > self.minute ||
            (other.minute == self.minute &&
                (other.second > self.second ||
                    (other.second == self.second &&
                        other.frame > self.frame)))
    }

    fn le(&self, other: &Self) -> bool {
        self < other || self == other
    }

    fn gt(&self, other: &Self) -> bool {
        other.minute < self.minute ||
            (other.minute == self.minute &&
                (other.second < self.second ||
                    (other.second == self.second &&
                        other.frame < self.frame)))
    }

    fn ge(&self, other: &Self) -> bool {
        self > other || self == other
    }
}

impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Timestamp with minute {}, second {}, and frame {}",
            self.minute, self.second, self.frame
        )
    }
}
