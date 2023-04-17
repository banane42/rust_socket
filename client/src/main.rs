use std::net::{UdpSocket, SocketAddr};
use std::{str,io};

//Client
fn main() {

      let local_host = [127, 0, 0, 1];

      //Constructs 10 addressess at local host, starting from port 6900 and going up by 1
      let addrs: [SocketAddr; 10] = core::array::from_fn( |i| {
            let port: u16 = u16::try_from(6900 + i).expect("Could not convert usize to u16");   
            SocketAddr::from((local_host, port))
      });

      let socket = UdpSocket::bind(&addrs[..])
                              .expect("Could not bind client socket");
      socket.connect("127.0.0.1:8888")
            .expect("Could not connect to server");
      loop {
            let mut input = String::new();
            let mut buffer = [0u8; 1500];
            io::stdin().read_line(&mut input)
                        .expect("Failed to read from stdin");
            socket.send(input.as_bytes())
                  .expect("Failed to write to server");

            socket.recv_from(&mut buffer)
                  .expect("Could not read into buffer");
            print!("{}", str::from_utf8(&buffer)
                              .expect("Could not write buffer as string"));
      }
}
