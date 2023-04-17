use std::collections::HashMap;
use std::net::{UdpSocket, SocketAddr};

pub struct Server {
    pub connections: HashMap<SocketAddr, SocketAddr>,
    pub socket: UdpSocket
}

impl Server {
    pub fn new(addr: &str) -> Result<Self, std::io::Error> {
        
        let socket = UdpSocket::bind(addr)?;

        Ok(Self {
            connections: HashMap::new(),
            socket,
        })
    }

    pub fn send() {
        let mut buf = [0u8; 1500];
    }

}