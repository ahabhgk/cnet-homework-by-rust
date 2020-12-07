mod smtp;
mod udp_ping;
mod web_server;

use std::{env, thread};

fn main() {
    let mut args = env::args();
    args.next();
    let error_msg = "Need a lab number...";
    let lab_num = match args.next() {
        Some(arg) => arg.parse::<i32>().expect(error_msg),
        None => panic!(error_msg),
    };
    match lab_num {
        1 => web_server::server::start().unwrap(),
        2 => {
            thread::spawn(|| {
                udp_ping::server::start();
            });
            let client_thread = thread::spawn(|| {
                udp_ping::client::start();
            });
            client_thread.join().unwrap();
            println!("client closed, then close the server by return from `main`");
        }
        3 => smtp::client::start(),
        _ => panic!("Stay turned..."),
    }
}
