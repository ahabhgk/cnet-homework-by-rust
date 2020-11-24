use rand;
use rand::Rng;
use std::net::UdpSocket;

pub fn start() {
    let socket = UdpSocket::bind("127.0.0.1:7878").unwrap();

    loop {
        let mut buffer = vec![0; 1024];
        let (amount, addr) = socket.recv_from(&mut buffer).unwrap();

        let num = rand::thread_rng().gen_range(0, 10);
        if num < 4 {
            continue;
        }

        let buffer = buffer[..amount].to_vec();
        let message = String::from_utf8(buffer).unwrap();
        let message = message.to_uppercase();

        socket.send_to(message.as_bytes(), addr).unwrap();
    }
}
