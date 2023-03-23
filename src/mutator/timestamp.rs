use crate::utils::DEFAULT_FPS;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Eq)]
pub struct TimeStamp {
    minute: u8,
    second: u8,
    frame: u8,
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
        if self.minute < other.minute {
            return true;
        } else if self.minute == other.minute {
            if self.second < other.minute {
                return true;
            } else if self.second == other.minute {
                if self.frame < other.frame {
                    return true;
                }
                return false;
            }
            return false;
        }
        false
    }

    fn le(&self, other: &Self) -> bool {
        self < other || self == other
    }

    fn gt(&self, other: &Self) -> bool {
        if self.minute > other.minute {
            return true;
        } else if self.minute == other.minute {
            if self.second > other.minute {
                return true;
            } else if self.second == other.minute {
                if self.frame > other.frame {
                    return true;
                }
                return false;
            }
            return false;
        }
        false
    }

    fn ge(&self, other: &Self) -> bool {
        self > other || self == other
    }
}
