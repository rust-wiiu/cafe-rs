use crate::prelude::*;

use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4},
};

pub trait ToSocketAddrs {
    type Iter: Iterator<Item = SocketAddrV4>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter>;
}

impl ToSocketAddrs for ([u8; 4], u16) {
    type Iter = std::iter::Once<SocketAddrV4>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        let addr = SocketAddrV4::new(self.0.into(), self.1);
        Ok(std::iter::once(addr))
    }
}

impl ToSocketAddrs for SocketAddr {
    type Iter = std::iter::Once<SocketAddrV4>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        match self {
            SocketAddr::V4(addr) => Ok(std::iter::once(*addr)),
            SocketAddr::V6(_) => Err(io::Error::AddrNotAvailable),
        }
    }
}

impl ToSocketAddrs for SocketAddrV4 {
    type Iter = std::iter::Once<SocketAddrV4>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        Ok(std::iter::once(*self))
    }
}

impl ToSocketAddrs for (IpAddr, u16) {
    type Iter = std::iter::Once<SocketAddrV4>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        match self.0 {
            IpAddr::V4(addr) => Ok(std::iter::once(SocketAddrV4::new(addr, self.1))),
            IpAddr::V6(_) => Err(io::Error::AddrNotAvailable),
        }
    }
}

impl ToSocketAddrs for (Ipv4Addr, u16) {
    type Iter = std::iter::Once<SocketAddrV4>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        Ok(std::iter::once(SocketAddrV4::new(self.0, self.1)))
    }
}
