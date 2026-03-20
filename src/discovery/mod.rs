use std::net::{Ipv4Addr, SocketAddr};

use tokio::net::UdpSocket;


struct Device {
    address: Ipv4Addr
}


async fn discover() {
    let socket = UdpSocket::bind("0.0.0.0:5253".parse::<SocketAddr>().unwrap());
}
