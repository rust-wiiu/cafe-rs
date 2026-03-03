//! socket

use crate::prelude::*;

use cafe::rrc::{Resource, Rrc};
use std::{io, mem::size_of_val, net::SocketAddrV4};
use sys::nsys::net::socket;

static SOCKET: Rrc = Rrc::new(
    || unsafe {
        let _ = socket::init();
    },
    || unsafe {
        let _ = socket::deinit();
    },
);

pub use socket::{Flags, Level, Options, Protocol, Shutdown};

#[derive(Debug, Clone)]
pub struct Socket {
    fd: socket::FileDescriptor,
    _resource: Resource,
}

impl Socket {
    #[inline]
    pub fn last_error() -> Option<socket::Error> {
        match unsafe { socket::last_error() } {
            socket::Error::Success => None,
            err => Some(err),
        }
    }

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
            Err(Self::last_error()
                .expect("Error does not reflect failed operation")
                .into())
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
            Err(Self::last_error()
                .expect("Error does not reflect failed operation")
                .into())
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
            Err(Self::last_error()
                .expect("Error does not reflect failed operation")
                .into())
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
            Err(Self::last_error()
                .expect("Error does not reflect failed operation")
                .into())
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn listen(&self, backlog: i32) -> io::Result<()> {
        let x = unsafe { socket::listen(self.fd, backlog) };

        if x == -1 {
            Err(Self::last_error()
                .expect("Error does not reflect failed operation")
                .into())
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn accept(&self) -> io::Result<(Socket, SocketAddrV4)> {
        let mut addr = std::mem::MaybeUninit::zeroed();
        let mut addrlen = size_of_val(&addr);

        let socket = unsafe { socket::accept(self.fd, addr.as_mut_ptr(), &mut addrlen) };
        debug_assert_eq!(addrlen, size_of_val(&addr));

        if socket == -1 {
            Err(Self::last_error()
                .expect("Error does not reflect failed operation")
                .into())
        } else {
            let addr = unsafe { addr.assume_init() };

            Ok((
                Self {
                    fd: socket,
                    _resource: self._resource.clone(),
                },
                SocketAddrV4::new(addr.address.into(), addr.port),
            ))
        }
    }

    #[inline]
    pub fn recv(&self, buffer: &mut [u8], flags: Option<Flags>) -> io::Result<usize> {
        let len = buffer.len() as u32;
        let buffer = buffer.as_mut_ptr();
        let flags = flags.unwrap_or_default();

        let received = unsafe { socket::recv(self.fd, buffer.cast(), len, flags) };

        if received == -1 {
            Err(Self::last_error()
                .expect("Error does not reflect failed operation")
                .into())
        } else {
            Ok(received as usize)
        }
    }

    #[inline]
    pub fn recvfrom(
        &self,
        buffer: &mut [u8],
        flags: Option<Flags>,
    ) -> io::Result<(usize, SocketAddrV4)> {
        let len = buffer.len() as u32;
        let buffer = buffer.as_mut_ptr();
        let flags = flags.unwrap_or_default();

        let mut from = std::mem::MaybeUninit::zeroed();
        let mut fromlen = size_of_val(&from);

        let received = unsafe {
            socket::recvfrom(
                self.fd,
                buffer.cast(),
                len,
                flags,
                from.as_mut_ptr(),
                &mut fromlen,
            )
        };
        debug_assert_eq!(fromlen, size_of_val(&from));

        if received == -1 {
            Err(Self::last_error()
                .expect("Error does not reflect failed operation")
                .into())
        } else {
            let from = unsafe { from.assume_init() };
            Ok((
                received as usize,
                SocketAddrV4::new(from.address.into(), from.port),
            ))
        }
    }

    #[inline]
    pub fn send(&self, buffer: &[u8], flags: Option<Flags>) -> io::Result<usize> {
        let len = buffer.len() as u32;
        let buffer = buffer.as_ptr();
        let flags = flags.unwrap_or_default();

        let sent = unsafe { socket::send(self.fd, buffer.cast(), len, flags) };

        if sent == -1 {
            Err(match Self::last_error() {
                None => io::Error::ConnectionAborted,
                Some(err) => err.into(),
            })
        } else {
            Ok(sent as usize)
        }
    }

    #[inline]
    pub fn sendto(
        &self,
        buffer: &[u8],
        to: SocketAddrV4,
        flags: Option<Flags>,
    ) -> io::Result<usize> {
        let len = buffer.len() as u32;
        let buffer = buffer.as_ptr();
        let flags = flags.unwrap_or_default();

        let addr = convert_addr(to);
        let addrlen = size_of_val(&addr);

        let sent = unsafe { socket::sendto(self.fd, buffer.cast(), len, flags, &addr, addrlen) };

        if sent == -1 {
            Err(Self::last_error()
                .expect("Error does not reflect failed operation")
                .into())
        } else {
            Ok(sent as usize)
        }
    }

    #[inline]
    pub fn peer(&self) -> io::Result<SocketAddrV4> {
        let mut addr = std::mem::MaybeUninit::zeroed();
        let mut addrlen = size_of_val(&addr);

        let x = unsafe { socket::get_peername(self.fd, addr.as_mut_ptr(), &mut addrlen) };
        debug_assert_eq!(addrlen, size_of_val(&addr));

        if x == -1 {
            Err(Self::last_error()
                .expect("Error does not reflect failed operation")
                .into())
        } else {
            let addr = unsafe { addr.assume_init() };
            Ok(SocketAddrV4::new(addr.address.into(), addr.port))
        }
    }

    #[inline]
    pub fn local(&self) -> io::Result<SocketAddrV4> {
        let mut addr = std::mem::MaybeUninit::zeroed();
        let mut addrlen = size_of_val(&addr);

        let x = unsafe { socket::get_sockname(self.fd, addr.as_mut_ptr(), &mut addrlen) };
        debug_assert_eq!(addrlen, size_of_val(&addr));

        if x == -1 {
            Err(Self::last_error()
                .expect("Error does not reflect failed operation")
                .into())
        } else {
            let addr = unsafe { addr.assume_init() };
            Ok(SocketAddrV4::new(addr.address.into(), addr.port))
        }
    }

    #[inline]
    pub fn set_options<T: Copy>(
        &self,
        level: socket::Level,
        option: socket::Options,
        value: T,
    ) -> io::Result<()> {
        let opt = &value as *const T;
        let optlen = size_of_val(&value);

        let x = unsafe { socket::setsockopt(self.fd, level, option, opt.cast(), optlen) };

        if x == -1 {
            Err(Self::last_error()
                .expect("Error does not reflect failed operation")
                .into())
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn get_options<T: Copy + Default>(
        &self,
        level: socket::Level,
        option: socket::Options,
    ) -> io::Result<T> {
        let mut opt = T::default();
        let mut optlen = size_of_val(&opt);

        let x = unsafe {
            socket::getsockopt(
                self.fd,
                level,
                option,
                &mut opt as *mut T as *mut _,
                &mut optlen,
            )
        };
        debug_assert_eq!(optlen, size_of_val(&opt));

        if x == -1 {
            Err(Self::last_error()
                .expect("Error does not reflect failed operation")
                .into())
        } else {
            Ok(opt)
        }
    }

    #[inline]
    pub fn shutdown(&self, how: socket::Shutdown) -> io::Result<()> {
        let x = unsafe { socket::shutdown(self.fd, how) };

        if x == -1 {
            Err(Self::last_error()
                .expect("Error does not reflect failed operation")
                .into())
        } else {
            Ok(())
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { socket::close(self.fd) };
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
