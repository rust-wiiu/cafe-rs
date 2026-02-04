//! network

use crate::rrc::{Resource, Rrc};
use crate::sys::nn_ac;
use crate::{Error, Result};

static AC: Rrc = Rrc::new(
    || unsafe {
        match nn_ac::ac::init() {
            nn_ac::ac::AcStatus::Failed => panic!("auto connect could not initialize"),
            nn_ac::ac::AcStatus::Processing => (),
            nn_ac::ac::AcStatus::Ok => (),
        }
    },
    || unsafe { nn_ac::ac::deinit() },
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
        match unsafe { nn_ac::ac::connect() } {
            nn_ac::ac::AcStatus::Failed => Err(Error::Any("cannot connect to network")),
            nn_ac::ac::AcStatus::Ok => Ok(()),
            // cannot happen in non-async mode
            nn_ac::ac::AcStatus::Processing => Ok(()),
        }
    }

    fn _disconnect(&self) -> Result<()> {
        match unsafe { nn_ac::ac::close() } {
            nn_ac::ac::AcStatus::Failed => Err(Error::Any("cannot disconnect from network")),
            nn_ac::ac::AcStatus::Ok => Ok(()),
            // cannot happen in non-async mode
            nn_ac::ac::AcStatus::Processing => Ok(()),
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
