use base64::encode;
use dotenv;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn start() {
    let subject = "try smtp on rust";
    let content_type = "text/plain";
    let msg = "I love computer networks and rust!";

    let from_address = dotenv::var("FROM_ADDRESS").unwrap();
    let to_address = dotenv::var("TO_ADDRESS").unwrap();

    let username = dotenv::var("USERNAME").unwrap();
    let password = dotenv::var("PASSWORD").unwrap();

    let client_socket = TcpStream::connect("smtp.qq.com:25").unwrap();
    let mut stream = BufReader::new(client_socket);
    println!("Connect");

    let code = read_code(&mut stream);
    if code != "220" {
        panic!("220 reply not received from server, code: {}.", code);
    }

    stream.get_mut().write(b"HELO smtp_qq\r\n").unwrap();
    stream.get_mut().flush().unwrap();
    read_code(&mut stream);
    read_code(&mut stream);
    let code = read_code(&mut stream);
    if code != "250" {
        panic!("250 reply not received from server, code: {}.", code);
    }

    stream.get_mut().write(b"AUTH LOGIN\r\n").unwrap();
    stream.get_mut().flush().unwrap();
    let code = read_code(&mut stream);
    if code != "334" {
        panic!("334 reply not received from server, code: {}.", code);
    }

    stream
        .get_mut()
        .write((encode(username) + "\r\n").as_bytes())
        .unwrap();
    stream.get_mut().flush().unwrap();
    let code = read_code(&mut stream);
    if code != "334" {
        panic!("334 reply not received from server, code: {}.", code);
    }

    stream
        .get_mut()
        .write((encode(password) + "\r\n").as_bytes())
        .unwrap();
    stream.get_mut().flush().unwrap();
    let code = read_code(&mut stream);
    if code != "235" {
        panic!("235 reply not received from server, code: {}.", code);
    }

    stream
        .get_mut()
        .write(format!("MAIL FROM: <{}>\r\n", from_address).as_bytes())
        .unwrap();
    stream.get_mut().flush().unwrap();
    let code = read_code(&mut stream);
    if code != "250" {
        panic!("250 reply not received from server, code: {}.", code);
    }

    stream
        .get_mut()
        .write(format!("RCPT TO: <{}>\r\n", to_address).as_bytes())
        .unwrap();
    stream.get_mut().flush().unwrap();
    let code = read_code(&mut stream);
    if code != "250" {
        panic!("250 reply not received from server, code: {}.", code);
    }

    stream.get_mut().write(b"DATA\r\n").unwrap();
    stream.get_mut().flush().unwrap();
    let code = read_code(&mut stream);
    if code != "354" {
        panic!("354 reply not received from server, code: {}.", code);
    }

    let message = format!(
        "from: {}\r\nto: {}\r\nsubject: {}\r\nContent-Type: {}\t\n\r\n{}\r\n.\r\n",
        from_address, to_address, subject, content_type, msg
    );

    stream.get_mut().write(message.as_bytes()).unwrap();
    stream.get_mut().flush().unwrap();
    let code = read_code(&mut stream);
    if code != "250" {
        panic!("250 reply not received from server, code: {}.", code);
    }

    stream.get_mut().write(b"QUIT\r\n").unwrap();
    stream.get_mut().flush().unwrap();
}

fn read_code(stream: &mut BufReader<TcpStream>) -> String {
    let mut recv = String::new();
    stream.read_line(&mut recv).unwrap();
    let code = &recv[0..3];
    String::from(code)
}
