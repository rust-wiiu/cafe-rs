//! network

use crate::prelude::*;
use sys::nn::ac;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Auto connect could not be initialized")]
    Initialization,
    #[error("Cannot connect to auto-configured network")]
    NetworkConnect,
    #[error("Cannot disconnect from network")]
    NetworkDisconnect,
}

pub fn init() -> Result<(), Error> {
    match unsafe { ac::init() } {
        ac::AcStatus::Failed => Err(Error::Initialization),
        ac::AcStatus::Processing | ac::AcStatus::Ok => Ok(()),
    }
}

pub fn deinit() {
    unsafe { ac::deinit() }
}

pub fn connect() -> Result<(), Error> {
    match unsafe { ac::connect() } {
        ac::AcStatus::Failed => Err(Error::NetworkConnect),
        ac::AcStatus::Processing | ac::AcStatus::Ok => Ok(()),
    }
}

pub fn disconnect() -> Result<(), Error> {
    match unsafe { ac::close() } {
        ac::AcStatus::Failed => Err(Error::NetworkDisconnect),
        ac::AcStatus::Processing | ac::AcStatus::Ok => Ok(()),
    }
}
