use chrono::Utc;
use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use std::time;

pub fn start() {
    let socket = UdpSocket::bind("127.0.0.1:7879").unwrap();
    socket
        .set_read_timeout(Some(time::Duration::new(1, 0)))
        .unwrap();
    let server_addr = SocketAddr::from_str("127.0.0.1:7878").unwrap();

    for num in 1..11 {
        let send_time = Utc::now();
        let message = format!("ping {} {}", num, send_time);
        socket.send_to(message.as_bytes(), server_addr).unwrap();

        let mut buffer = vec![0; 1024];
        match socket.recv_from(&mut buffer) {
            Ok((_, addr)) => {
                if addr == server_addr {
                    let recv_time = Utc::now();
                    let rtt = recv_time - send_time;
                    println!("Sequence {}: Reply from {}    RTT = {}", num, addr, rtt);
                } else {
                    println!("Sequence {}: Reply from wrong addr {}...", num, addr);
                }
            }
            Err(_) => println!("Sequence {}: Request timed out...", num),
        };
    }
}
