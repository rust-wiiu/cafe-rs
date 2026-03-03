use crate::prelude::*;

use super::socket::{Flags, Level, Options, Socket};
use std::{
    io,
    net::{SocketAddrV4, ToSocketAddrs},
    time::Duration,
};

#[derive(Debug)]
pub struct UdpSocket {
    socket: Socket,
}

impl UdpSocket {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = Socket::udp()?;

        let mut err = io::Error::AddrNotAvailable;
        for addr in addr.to_socket_addrs()? {
            match socket.bind(addr) {
                Ok(_) => return Ok(Self { socket }),
                Err(e) => err = e,
            }
        }
        Err(err)
    }

    pub fn connect<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()> {
        let mut err = io::Error::AddrNotAvailable;
        for addr in addr.to_socket_addrs()? {
            match self.socket.connect(addr) {
                Ok(_) => return Ok(()),
                Err(e) => err = e,
            }
        }
        Err(err)
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddrV4)> {
        self.socket.recvfrom(buf, None)
    }

    pub fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddrV4)> {
        self.socket.recvfrom(buf, Some(Flags::Peek))
    }

    pub fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], addr: A) -> io::Result<usize> {
        let mut err = io::Error::AddrNotAvailable;
        for addr in addr.to_socket_addrs()? {
            match self.socket.sendto(buf, addr, None) {
                Ok(n) => return Ok(n),
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

    // pub fn try_clone

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

    pub fn set_broadcast(&self, broadcast: bool) -> io::Result<()> {
        self.socket
            .set_options(Level::Socket, Options::Broadcast, broadcast as u32)
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        match self
            .socket
            .get_options::<u32>(Level::Socket, Options::Broadcast)
        {
            Ok(v) if v == 1 => Ok(true),
            Ok(v) if v == 0 => Ok(false),
            Ok(_) => Err(io::Error::InvalidData),
            Err(err) => Err(err),
        }
    }

    // pub fn set_multicast_loop_v4
    // pub fn multicast_loop_v4
    // pub fn set_multicast_ttl_v4
    // pub fn multicast_ttl_v4

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.socket.set_options(Level::Ip, Options::TimeToLive, ttl)
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.socket.get_options(Level::Ip, Options::TimeToLive)
    }

    // pub fn join_multicast_v4
    // pub fn leave_multicast_v4

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(Socket::last_error().map(Into::into))
    }

    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.socket.send(buf, None)
    }

    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.recv(buf, None)
    }

    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.recv(buf, Some(Flags::Peek))
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.socket
            .set_options(Level::Socket, Options::NonBlocking, nonblocking as i32)
    }
}
