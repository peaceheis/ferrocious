use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt;
use std::fmt::Formatter;
use vulkano::buffer::BufferContents;

#[derive(PartialEq, Eq, Debug, Default, Copy, Clone, BufferContents)]
#[repr(C)]
pub struct TimeStamp {
    pub minute: u32,
    pub second: u32,
    pub frame: u32,
}

impl TimeStamp {
    pub fn new(minute: u32, second: u32, frame: u32) -> Self {
        TimeStamp {
            minute,
            second,
            frame,
        }
    }

    pub fn as_num_frames(&self, fps: u32) -> u32 {
        (self.minute * 60 * fps + self.second * fps + self.frame) as u32
    }

    pub fn new_with_defaults(minute: Option<u32>, second: Option<u32>, frame: Option<u32>) -> Self {
        TimeStamp {
            minute: minute.unwrap_or(0),
            second: second.unwrap_or(0),
            frame: frame.unwrap_or(0),
        }
    }

    pub fn increment(&mut self, fps: u32) {
        self.frame += 1;

        if self.frame >= fps.into() {
            self.frame = 0;
            self.second += 1;
        }

        if self.second > 59 {
            self.second = 0;
            self.minute += 1;
        }
    }

    pub fn time_as_array(&self) -> [u32; 3] {
        [self.minute, self.second, self.frame]
    }

    pub fn in_range(&self, intervals: &Vec<[TimeStamp; 2]>) -> bool {
        intervals
            .iter()
            .any(|interval| interval[0] <= *self && *self <= interval[1])
    }

    pub fn matches_range(&self, range: &Option<Vec<[TimeStamp; 2]>>) -> bool {
        match range {
            None => true, // No range means always visible
            Some(intervals) => self.in_range(intervals),
        }
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
        other.minute > self.minute
            || (other.minute == self.minute
                && (other.second > self.second
                    || (other.second == self.second && other.frame > self.frame)))
    }

    fn le(&self, other: &Self) -> bool {
        self < other || self == other
    }

    fn gt(&self, other: &Self) -> bool {
        other.minute < self.minute
            || (other.minute == self.minute
                && (other.second < self.second
                    || (other.second == self.second && other.frame < self.frame)))
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
