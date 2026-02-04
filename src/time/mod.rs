//! time

use crate::sys::coreinit::time;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemTime(pub(crate) time::Time);

impl SystemTime {
    /// The current local time.
    ///
    /// Local time is the time configured in the system settings.
    pub fn now() -> Self {
        Self(unsafe { time::local_time() })
    }

    ///The current system uptime.
    ///
    /// The OS sets this value to the `00:00 AM January 1, 2000` on boot. Application changes might cause reboots.
    pub fn uptime() -> Self {
        Self(unsafe { time::system_uptime() })
    }
}
