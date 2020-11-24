use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;

pub fn server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                let mut message = vec![0; 1024];
                s.read(&mut message).unwrap();

                let filename = String::from_utf8(message).unwrap();
                let filename = filename.split_whitespace().collect::<Vec<&str>>()[1];
                let base_path = "./assets/";
                let path = format!("{}{}", base_path, filename);
                let (contents, status_line) = if let Ok(c) = fs::read_to_string(path) {
                    (c, "HTTP/1.1 200 OK\r\n")
                } else {
                    (
                        fs::read_to_string(base_path.to_owned() + "404.html").unwrap(),
                        "HTTP/1.1 404 Not Found\r\n",
                    )
                };

                let response = format!(
                    "{}Content-Length: {}\r\n\r\n{}",
                    status_line,
                    contents.len(),
                    contents
                );

                s.write(response.as_bytes()).unwrap();
                s.flush().unwrap();
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }
}
