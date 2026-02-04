//! Datetime

use crate::std::{fmt, time::SystemTime};
use crate::sys::coreinit::time;

/// DateTime
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateTime(time::DateTime);

impl DateTime {
    pub fn now() -> Self {
        let time = SystemTime::now();
        Self::from(time)
    }

    /// Seconds after the minute from 0 to 59.
    pub fn second(&self) -> u32 {
        self.0.sec as u32
    }

    /// Returns the minute number from 0 to 59.
    pub fn minute(&self) -> u32 {
        self.0.min as u32
    }

    /// Returns the hour number from 0 to 23.
    pub fn hour(&self) -> u32 {
        self.0.hour as u32
    }

    /// Returns the day of the month from 1 to 31.
    pub fn day_of_month(&self) -> u32 {
        self.0.mday as u32
    }

    /// Returns the month since January from 0 to 11.
    pub fn month(&self) -> u32 {
        self.0.mon as u32
    }

    /// Returns the year since 0 AD.
    pub fn year(&self) -> u32 {
        self.0.year as u32
    }

    /// Returns the day of the week since Sunday from 0 to 6.
    pub fn weekday(&self) -> u32 {
        self.0.wday as u32
    }

    /// Returns the day of the year from 0 to 365.
    pub fn ordinal(&self) -> u32 {
        self.0.yday as u32
    }

    /// Returns the milliseconds after the second from 0 to 999.
    pub fn millisecond(&self) -> u32 {
        self.0.msec as u32
    }

    /// Returns the microseconds after the millisecond from 0 to 999.
    pub fn nanosecond(&self) -> u32 {
        self.0.usec as u32
    }
}

impl From<SystemTime> for DateTime {
    fn from(value: SystemTime) -> Self {
        let mut dt = time::DateTime::default();
        unsafe {
            time::time_to_datetime(value.0, &mut dt);
        }
        Self(dt)
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
            self.year(),
            self.month() + 1,
            self.day_of_month(),
            self.hour(),
            self.minute(),
            self.second(),
            self.millisecond()
        )
    }
}
