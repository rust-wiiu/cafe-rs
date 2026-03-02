//! socket

use crate::prelude::*;

use cafe::rrc::{Resource, Rrc};
use std::{io, mem::size_of_val, net::SocketAddrV4, ptr};
use sys::nsys::net::socket;

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
    fd: socket::FileDescriptor,
    _resource: Resource,
}

impl Socket {
    #[inline]
    pub fn tcp() -> io::Result<Self> {
        let _resource = SOCKET.acquire();

        let fd = unsafe {
            socket::socket(
                socket::Family::IPv4,
                socket::Type::Stream,
                socket::Protocol::Tcp,
            )
        };

        if fd == -1 {
            Err(unsafe { socket::last_error() }.into())
        } else {
            Ok(Self { fd, _resource })
        }
    }

    #[inline]
    pub fn udp() -> io::Result<Self> {
        let _resource = SOCKET.acquire();

        let fd = unsafe {
            socket::socket(
                socket::Family::IPv4,
                socket::Type::Datagram,
                socket::Protocol::Udp,
            )
        };

        if fd == -1 {
            Err(unsafe { socket::last_error() }.into())
        } else {
            Ok(Self { fd, _resource })
        }
    }

    #[inline]
    pub fn bind(&self, addr: SocketAddrV4) -> io::Result<()> {
        let addr = convert_addr(addr);
        let addrlen = size_of_val(&addr);

        let x = unsafe { socket::bind(self.fd, &addr, addrlen) };

        if x == -1 {
            Err(unsafe { socket::last_error() }.into())
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn connect(&self, remote: SocketAddrV4) -> io::Result<()> {
        let addr = convert_addr(remote);
        let addrlen = size_of_val(&addr);

        let x = unsafe { socket::connect(self.fd, &addr, addrlen) };

        if x == -1 {
            Err(unsafe { socket::last_error() }.into())
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn listen(&self, backlog: i32) -> io::Result<()> {
        let x = unsafe { socket::listen(self.fd, backlog) };

        if x == -1 {
            Err(unsafe { socket::last_error() }.into())
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn recv(&self, buffer: &mut [u8], flags: Option<socket::Flags>) -> io::Result<usize> {
        let len = buffer.len() as u32;
        let buffer = buffer.as_mut_ptr();
        let flags = flags.unwrap_or_default();

        let received = unsafe { socket::recv(self.fd, buffer.cast(), len, flags) };

        if received == -1 {
            Err(unsafe { socket::last_error() }.into())
        } else {
            Ok(received as usize)
        }
    }

    #[inline]
    pub fn recvfrom(
        &self,
        buffer: &mut [u8],
        flags: Option<socket::Flags>,
        from: Option<SocketAddrV4>,
    ) -> io::Result<usize> {
        let len = buffer.len() as u32;
        let buffer = buffer.as_mut_ptr();
        let flags = flags.unwrap_or_default();

        let addr = from.map(convert_addr);

        let (from, fromlen) = if let Some(addr) = &addr {
            (addr as *const socket::Address, size_of_val(addr))
        } else {
            (ptr::null(), 0)
        };

        let received =
            unsafe { socket::recvfrom(self.fd, buffer.cast(), len, flags, from, fromlen) };

        if received == -1 {
            Err(unsafe { socket::last_error() }.into())
        } else {
            Ok(received as usize)
        }
    }

    #[inline]
    pub fn send(&self, buffer: &[u8], flags: Option<socket::Flags>) -> io::Result<usize> {
        let len = buffer.len() as u32;
        let buffer = buffer.as_ptr();
        let flags = flags.unwrap_or_default();

        let sent = unsafe { socket::send(self.fd, buffer.cast(), len, flags) };

        if sent == -1 {
            Err(unsafe { socket::last_error() }.into())
        } else {
            Ok(sent as usize)
        }
    }

    #[inline]
    pub fn sendto(
        &self,
        buffer: &[u8],
        to: SocketAddrV4,
        flags: Option<socket::Flags>,
    ) -> io::Result<usize> {
        let len = buffer.len() as u32;
        let buffer = buffer.as_ptr();
        let flags = flags.unwrap_or_default();

        let addr = convert_addr(to);
        let addrlen = size_of_val(&addr);

        let sent = unsafe { socket::sendto(self.fd, buffer.cast(), len, flags, &addr, addrlen) };

        if sent == -1 {
            Err(unsafe { socket::last_error() }.into())
        } else {
            Ok(sent as usize)
        }
    }

    #[inline]
    pub fn peer(&self) -> io::Result<SocketAddrV4> {
        let mut addr = socket::Address::default();
        let addrlen = size_of_val(&addr);

        let x = unsafe { socket::get_peername(self.fd, &mut addr, addrlen) };

        if x == -1 {
            Err(unsafe { socket::last_error() }.into())
        } else {
            Ok(SocketAddrV4::new(addr.address.into(), addr.port))
        }
    }

    #[inline]
    pub fn address(&self) -> io::Result<SocketAddrV4> {
        let mut addr = socket::Address::default();
        let addrlen = size_of_val(&addr);

        let x = unsafe { socket::get_sockname(self.fd, &mut addr, addrlen) };

        if x == -1 {
            Err(unsafe { socket::last_error() }.into())
        } else {
            Ok(SocketAddrV4::new(addr.address.into(), addr.port))
        }
    }

    #[inline]
    pub fn shutdown(&self, how: socket::Shutdown) -> io::Result<()> {
        let x = unsafe { socket::shutdown(self.fd, how) };

        if x == -1 {
            Err(unsafe { socket::last_error() }.into())
        } else {
            Ok(())
        }
    }

    #[inline]
    fn close(&self) -> io::Result<()> {
        let x = unsafe { socket::close(self.fd) };

        if x == -1 {
            Err(unsafe { socket::last_error() }.into())
        } else {
            Ok(())
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        let _ = self.shutdown(socket::Shutdown::ReadWrite);
        let _ = self.close();
    }
}

#[inline]
const fn convert_addr(addr: SocketAddrV4) -> socket::Address {
    socket::Address {
        family: socket::Family::IPv4,
        port: addr.port(),
        address: addr.ip().to_bits(),
        zero: [0; 8],
    }
}
