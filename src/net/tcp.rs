use crate::prelude::*;

use super::socket::{Flags, Level, Options, Shutdown, Socket};
use std::{
    io,
    iter::FusedIterator,
    net::{SocketAddrV4, ToSocketAddrs},
    time::Duration,
};

#[derive(Debug)]
pub struct TcpStream {
    socket: Socket,
}

#[derive(Debug)]
pub struct TcpListener {
    socket: Socket,
}

#[must_use]
#[derive(Debug)]
pub struct Incoming<'a> {
    listener: &'a TcpListener,
}

#[derive(Debug)]
pub struct IntoIncoming {
    listener: TcpListener,
}

impl TcpStream {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = Socket::tcp()?;

        let mut err = io::Error::AddrNotAvailable;
        for addr in addr.to_socket_addrs()? {
            match socket.connect(addr) {
                Ok(()) => return Ok(Self { socket }),
                Err(e) => err = e,
            }
        }
        Err(err)
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddrV4> {
        self.socket.peer()
    }

    pub fn local_addr(&self) -> io::Result<SocketAddrV4> {
        self.socket.local()
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.socket.shutdown(how)
    }

    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        let dur = dur
            .map(|d| (d.as_secs() as u32, d.as_millis() as u32))
            .unwrap_or_default();

        self.socket
            .set_options(Level::Socket, Options::ReceiveTimeout, dur)
    }

    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        let dur = dur
            .map(|d| (d.as_secs() as u32, d.as_millis() as u32))
            .unwrap_or_default();

        self.socket
            .set_options(Level::Socket, Options::SendTimeout, dur)
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        let dur: (u32, u32) = self
            .socket
            .get_options(Level::Socket, Options::ReceiveTimeout)?;

        if dur == (0, 0) {
            Ok(None)
        } else {
            Ok(Some(
                Duration::from_secs(dur.0 as u64) + Duration::from_millis(dur.1 as u64),
            ))
        }
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        let dur: (u32, u32) = self
            .socket
            .get_options(Level::Socket, Options::SendTimeout)?;

        if dur == (0, 0) {
            Ok(None)
        } else {
            Ok(Some(
                Duration::from_secs(dur.0 as u64) + Duration::from_millis(dur.1 as u64),
            ))
        }
    }

    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.recv(buf, Some(Flags::Peek))
    }

    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        self.socket
            .set_options(Level::Tcp, Options::NoDelay, nodelay as u32)
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        let nodelay: u32 = self.socket.get_options(Level::Tcp, Options::NoDelay)?;

        match nodelay {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(io::Error::InvalidData),
        }
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.socket.set_options(Level::Ip, Options::TimeToLive, ttl)
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.socket.get_options(Level::Ip, Options::TimeToLive)
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(Socket::last_error().map(Into::into))
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.socket
            .set_options(Level::Socket, Options::NonBlocking, nonblocking as i32)
    }
}

impl io::Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.socket.send(buf, None)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.recv(buf, None)
    }
}

impl io::Write for &TcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.socket.send(buf, None)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Read for &TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.recv(buf, None)
    }
}

impl TcpListener {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = Socket::tcp()?;

        let mut err = io::Error::AddrNotAvailable;
        for addr in addr.to_socket_addrs()? {
            match socket.bind(addr) {
                Ok(()) => {
                    socket.listen(128)?;
                    return Ok(Self { socket });
                }
                Err(e) => err = e,
            }
        }
        Err(err)
    }

    pub fn local_addr(&self) -> io::Result<SocketAddrV4> {
        self.socket.local()
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddrV4)> {
        self.socket
            .accept()
            .map(|(socket, addr)| (TcpStream { socket }, addr))
    }

    pub fn incoming(&self) -> Incoming<'_> {
        Incoming { listener: self }
    }

    #[must_use]
    pub fn into_incoming(self) -> IntoIncoming {
        IntoIncoming { listener: self }
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.socket.set_options(Level::Ip, Options::TimeToLive, ttl)
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.socket.get_options(Level::Ip, Options::TimeToLive)
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(Socket::last_error().map(Into::into))
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.socket
            .set_options(Level::Socket, Options::NonBlocking, nonblocking as i32)
    }
}

impl<'a> Iterator for Incoming<'a> {
    type Item = io::Result<TcpStream>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.listener.accept().map(|p| p.0))
    }
}

impl FusedIterator for Incoming<'_> {}

impl Iterator for IntoIncoming {
    type Item = io::Result<TcpStream>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.listener.accept().map(|p| p.0))
    }
}

impl FusedIterator for IntoIncoming {}
