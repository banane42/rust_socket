use std::collections::HashMap;
use std::net::{UdpSocket};

//Server
fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888")
                           .expect("Could not bind socket");

    let mut connections: HashMap<String, u8> = HashMap::new();

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("Failed to clone socket");
        
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {

                let addr_str = src.to_string();

                if !connections.contains_key(&addr_str) {
                    println!("New message from {}", &src);
                    connections.insert(addr_str, 6);
                }

                for (conn, _) in connections.iter() {
                    if !conn.to_string().eq(&src.to_string()) {
                        match sock.send_to(&buf, conn) {
                            Ok(_) => {},
                            Err(e) => {
                                eprint!("couldnt send to addr {}\nError: {}", conn, e);
                            }
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("couldn't recieve a datagram: {}", e);
            }
        }

    }
}