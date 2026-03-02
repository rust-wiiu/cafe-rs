//! network

use crate::rrc::{Resource, Rrc};
use crate::sys::nn::ac;
use crate::{Error, Result};

static AC: Rrc = Rrc::new(
    || unsafe {
        match ac::init() {
            ac::AcStatus::Failed => panic!("auto connect could not initialize"),
            ac::AcStatus::Processing => (),
            ac::AcStatus::Ok => (),
        }
    },
    || unsafe { ac::deinit() },
);

pub struct Network {
    _resource: Resource,
}

impl Default for Network {
    fn default() -> Self {
        Network {
            _resource: AC.acquire(),
        }
    }
}

impl Network {
    pub fn connect(&self) -> Result<()> {
        match unsafe { ac::connect() } {
            ac::AcStatus::Failed => Err(Error::Any("cannot connect to network")),
            ac::AcStatus::Ok => Ok(()),
            // cannot happen in non-async mode
            ac::AcStatus::Processing => Ok(()),
        }
    }

    fn _disconnect(&self) -> Result<()> {
        match unsafe { ac::close() } {
            ac::AcStatus::Failed => Err(Error::Any("cannot disconnect from network")),
            ac::AcStatus::Ok => Ok(()),
            // cannot happen in non-async mode
            ac::AcStatus::Processing => Ok(()),
        }
    }

    pub fn disconnect(self) -> Result<()> {
        self._disconnect()
    }
}

impl Drop for Network {
    fn drop(&mut self) {
        let _ = self._disconnect();
    }
}
