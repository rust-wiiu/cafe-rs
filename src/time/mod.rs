//! time

use crate::prelude::*;
use crate::{Error, Result};

use std::{ops, time::Duration};
use sys::coreinit;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemTime(pub(crate) coreinit::time::Time);

impl From<coreinit::time::Time> for SystemTime {
    fn from(value: coreinit::time::Time) -> Self {
        Self(value)
    }
}

impl From<Duration> for SystemTime {
    fn from(value: Duration) -> Self {
        // the max duration is over 500 years, so this should be fine
        let nsec = (value.as_nanos().min(u64::MAX as u128)) as u64;
        let timer_clock = unsafe { (*coreinit::system::system_info()).bus_clock_speed } / 4;

        Self((nsec * (timer_clock as u64) / 31250) / 32000)
    }
}

impl Into<Duration> for SystemTime {
    fn into(self) -> Duration {
        let timer_clock = unsafe { (*coreinit::system::system_info()).bus_clock_speed } / 4;

        let nanos = (self.0 as u128 * 31250 * 32000) / (timer_clock as u128);

        Duration::from_nanos((nanos.min(u64::MAX as u128)) as u64)
    }
}

impl Into<coreinit::time::Time> for SystemTime {
    fn into(self) -> coreinit::time::Time {
        self.0
    }
}

impl ops::Add<Duration> for SystemTime {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0 + Self::from(rhs).0)
    }
}

impl ops::AddAssign<Duration> for SystemTime {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Duration> for SystemTime {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        Self(self.0 - Self::from(rhs).0)
    }
}

impl ops::SubAssign<Duration> for SystemTime {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl SystemTime {
    pub const STARTUP: Self = SystemTime(0);

    /// The current local time.
    ///
    /// Local time is the time configured in the system settings.
    pub fn now() -> Self {
        Self(unsafe { coreinit::time::local_time() })
    }

    ///The current system uptime.
    ///
    /// The OS sets this value to the `00:00 AM January 1, 2000` on boot. Application changes might cause reboots.
    pub fn uptime() -> Self {
        Self(unsafe { coreinit::time::system_uptime() })
    }

    pub fn duration_since(&self, earlier: SystemTime) -> Result<Duration> {
        if earlier < *self {
            let diff = self.0 - earlier.0;
            Ok(Self(diff).into())
        } else {
            Err(Error::Any("earlier is later than now"))
        }
    }

    pub fn elapsed(&self) -> Result<Duration> {
        Self::now().duration_since(*self)
    }
}
