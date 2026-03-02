use crate::prelude::*;

use super::socket::Socket;
use std::{
    io,
    net::{SocketAddrV4, ToSocketAddrs},
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

    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.socket.send(buf, None)
    }

    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.recv(buf, None)
    }
}
