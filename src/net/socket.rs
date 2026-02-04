//! socket

use crate::rrc::{Resource, Rrc};
use crate::std::{mem::size_of_val, net::SocketAddrV4};
use crate::sys::nsysnet::socket;
use crate::{Error, Result};

static SOCKET: Rrc = Rrc::new(
    || unsafe {
        let _ = socket::init();
    },
    || unsafe {
        let _ = socket::deinit();
    },
);

#[derive(Debug, Clone)]
pub struct Socket {
    fd: socket::RawFd,
    _resource: Resource,
}

impl Socket {
    pub fn tcp() -> Result<Self> {
        let _resource = SOCKET.acquire();

        let fd = unsafe {
            socket::socket(
                socket::SocketFamily::IPv4,
                socket::SocketType::Stream,
                socket::SocketProtocol::Tcp,
            )
        };

        if fd == -1 {
            Err(Error::Any("socket returned -1"))
        } else {
            Ok(Self { fd, _resource })
        }
    }

    pub fn udp() -> Result<Self> {
        let _resource = SOCKET.acquire();

        let fd = unsafe {
            socket::socket(
                socket::SocketFamily::IPv4,
                socket::SocketType::Datagram,
                socket::SocketProtocol::Udp,
            )
        };

        if fd == -1 {
            Err(Error::Any("socket returned -1"))
        } else {
            Ok(Self { fd, _resource })
        }
    }

    pub fn sendto(
        &self,
        data: &[u8],
        to: &SocketAddrV4,
        flags: Option<socket::SocketFlags>,
    ) -> Result<()> {
        let to = (*to).into();

        let flags = flags.unwrap_or_default();

        let sent = unsafe {
            socket::sendto(
                self.fd,
                data.as_ptr().cast(),
                data.len() as i32,
                flags.bits(),
                &to,
                size_of_val(&to) as i32,
            )
        };

        if sent == -1 {
            let error = unsafe { socket::last_error() };
            Err(Error::Integer(error))
        } else {
            Ok(())
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe {
            socket::shutdown(self.fd, socket::Shutdown::ReadWrite);
            socket::close(self.fd);
        }
    }
}
