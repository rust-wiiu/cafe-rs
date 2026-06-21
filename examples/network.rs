#![no_std]
#![no_main]

use cafe::rt::rpx;
use cafe_rs::prelude::*;

rpx! {
    fn main() {
        cafe::net::network::init().unwrap();
        cafe::net::network::connect().unwrap();
        cafe::net::socket::init().unwrap();

        let socket = cafe::net::socket::Socket::udp().unwrap();

        let address = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 4405);

        for _ in 0..5 {
            socket.sendto(b"HELLO FROM WIIU\n", address, None).unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        cafe::net::socket::deinit().unwrap();
        cafe::net::network::disconnect().unwrap();
        cafe::net::network::deinit();
    }
}
