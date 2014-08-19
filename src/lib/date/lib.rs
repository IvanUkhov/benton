#![crate_name = "date"]
#![crate_type = "rlib"]

extern crate time;

#[deriving(Default, Eq, Ord, Show)]
pub struct Date {
    pub year: u32,
    pub month: u8,
    pub day: u8,
}

impl PartialEq for Date {
    fn eq(&self, other: &Date) -> bool {
        self.year == other.year &&
        self.month == other.month &&
        self.day == other.day
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        if self.year > other.year {
            return Some(std::cmp::Greater);
        }
        else if self.year < other.year {
            return Some(std::cmp::Less);
        }

        if self.month > other.month {
            return Some(std::cmp::Greater);
        }
        else if self.month < other.month {
            return Some(std::cmp::Less);
        }

        if self.day > other.day {
            return Some(std::cmp::Greater);
        }
        else if self.day < other.day {
            return Some(std::cmp::Less);
        }

        Some(std::cmp::Equal)
    }
}

impl Date {
    pub fn at(seconds: i64) -> Date {
        let time = time::at_utc(time::Timespec { sec: seconds, nsec: 0 });

        Date {
            year: (time.tm_year + 1900) as u32,
            month: (time.tm_mon + 1) as u8,
            day: time.tm_mday as u8,
        }
    }

    pub fn at_since_1904(seconds: i64) -> Date {
        Date::at(seconds - 2082844800)
    }
}
