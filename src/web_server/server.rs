use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::str::FromStr;

pub fn start() -> Result<(), Box<dyn Error>> {
    // server socket
    let addr = SocketAddr::from_str("127.0.0.1:7878")?;
    let listener = TcpListener::bind(addr)?;
    println!("Web server is running on {}...", addr);

    for stream in listener.incoming() {
        // connection socket
        let mut stream = stream?;
        let mut message = vec![0; 1024];
        stream.read(&mut message)?;

        let filename = String::from_utf8(message)?;
        let filename = filename.split_whitespace().collect::<Vec<&str>>()[1];
        let base_path = "./assets/";
        let path = format!("{}{}", base_path, filename);
        let (contents, status_line) = if let Ok(c) = fs::read_to_string(path) {
            (c, "HTTP/1.1 200 OK\r\n")
        } else {
            (
                fs::read_to_string(base_path.to_owned() + "404.html")?,
                "HTTP/1.1 404 Not Found\r\n",
            )
        };

        let response = format!(
            "{}Content-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write(response.as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}
