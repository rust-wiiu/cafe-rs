#![no_std]
#![no_main]

// use cafe_logger::CafeLogger;
use cafe_rs::prelude::*;
use cafe_rs::rt::rpx;

rpx! {
    fn main() -> i32 {
        // CafeLogger::default()
        //     .level(log::Level::Debug)
        //     .console(true)
        //     .init()
        //     .unwrap();

        let net = cafe_rs::net::network::Network::default();
        net.connect().unwrap();

        let socket = cafe_rs::net::socket::Socket::udp().unwrap();

        let address = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 4405);

        for _ in 0..5 {
            let now = cafe_rs::datetime::DateTime::now();
            log::info!("{}", now);

            socket.send_to(b"HELLO FROM WIIU\n", &address, None).unwrap();

            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        0
    }
}
