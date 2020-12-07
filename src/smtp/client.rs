use base64::encode;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::str::{from_utf8, Utf8Error};

pub fn start() {
    let subject = "try smtp on rust";
    let content_type = "text/plain";
    let msg = "I love computer networks and rust!";
    let end_msg = "\r\n.\r\n";

    let from_address = "1586790208@qq.com";
    let to_address = "ahabhgk@gmail.com";

    let username = "1586790208@qq.com";
    let password = "ipnnxiprobrwjafe";

    let mut client_socket = TcpStream::connect("smtp.qq.com:25").unwrap();
    let mut stream = BufReader::new(client_socket);
    println!("Connect");

    let code = read_code(&mut stream);
    if code != "220" {
        panic!("220 reply not received from server, code: {}.", code);
    }

    stream.get_mut().write(b"HELO smtp_qq\r\n");
    stream.get_mut().flush();
    read_code(&mut stream);
    read_code(&mut stream);
    let code = read_code(&mut stream);
    if code != "250" {
        panic!("250 reply not received from server, code: {}.", code);
    }

    stream.get_mut().write(b"AUTH LOGIN\r\n");
    stream.get_mut().flush();
    let code = read_code(&mut stream);
    if code != "334" {
        panic!("334 reply not received from server, code: {}.", code);
    }

    stream
        .get_mut()
        .write((encode(username) + "\r\n").as_bytes());
    stream.get_mut().flush();
    let code = read_code(&mut stream);
    if code != "334" {
        panic!("334 reply not received from server, code: {}.", code);
    }

    stream
        .get_mut()
        .write((encode(password) + "\r\n").as_bytes());
    stream.get_mut().flush();
    let code = read_code(&mut stream);
    if code != "235" {
        panic!("235 reply not received from server, code: {}.", code);
    }

    stream
        .get_mut()
        .write(format!("MAIL FROM: <{}>\r\n", from_address).as_bytes());
    stream.get_mut().flush();
    let code = read_code(&mut stream);
    if code != "250" {
        panic!("250 reply not received from server, code: {}.", code);
    }

    stream
        .get_mut()
        .write(format!("RCPT TO: <{}>\r\n", to_address).as_bytes());
    stream.get_mut().flush();
    let code = read_code(&mut stream);
    if code != "250" {
        panic!("250 reply not received from server, code: {}.", code);
    }

    stream.get_mut().write(b"DATA\r\n");
    stream.get_mut().flush();
    let code = read_code(&mut stream);
    if code != "354" {
        panic!("354 reply not received from server, code: {}.", code);
    }

    let message = format!(
        "from: {}\r\nto: {}\r\nsubject: {}\r\nContent-Type: {}\t\n\r\n{}\r\n.\r\n",
        from_address, to_address, subject, content_type, msg
    );

    stream.get_mut().write(message.as_bytes());
    stream.get_mut().flush();
    let code = read_code(&mut stream);
    if code != "250" {
        panic!("250 reply not received from server, code: {}.", code);
    }

    stream.get_mut().write(b"QUIT\r\n");
    stream.get_mut().flush();
}

fn read_code(stream: &mut BufReader<TcpStream>) -> String {
    let mut recv = String::new();
    stream.read_line(&mut recv);
    let code = &recv[0..3];
    String::from(code)
}
