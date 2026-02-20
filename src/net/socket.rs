//! socket

use crate::prelude::*;

use crate::{Error, Result};
use cafe::rrc::{Resource, Rrc};
use std::{
    mem::size_of_val,
    net::{SocketAddr, SocketAddrV4},
};
use sys::nsysnet::socket;

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

    pub fn send_to(
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

pub trait ToSocketAddrs {
    type Iter: Iterator<Item = SocketAddrV4>;

    fn to_socket_addrs(&self) -> Result<Self::Iter>;
}

impl ToSocketAddrs for ([u8; 4], u16) {
    type Iter = std::iter::Once<SocketAddrV4>;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        let addr = SocketAddrV4::new(self.0.into(), self.1);
        Ok(std::iter::once(addr))
    }
}

impl ToSocketAddrs for SocketAddr {
    type Iter = std::iter::Once<SocketAddrV4>;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        match self {
            SocketAddr::V4(addr) => Ok(std::iter::once(*addr)),
            SocketAddr::V6(_) => Err(Error::Any("IPv6 addresses are not supported")),
        }
    }
}

impl ToSocketAddrs for SocketAddrV4 {
    type Iter = std::iter::Once<SocketAddrV4>;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        Ok(std::iter::once(*self))
    }
}
