use std::net::{TcpListener, IpAddr};
use std::io::Write;
use std::time::Duration;
use std::thread;

pub mod lib;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6666").unwrap();
    let poll = lib::ThreadPoll::new(4);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        poll.execute(move || {
            let ip = stream.peer_addr().unwrap().ip();

            match ip {
                IpAddr::V4(ref a) => println!("ipv4 {}", a),
                IpAddr::V6(ref a) => println!("ipv6 {}", a),
            }

            thread::sleep(Duration::from_secs(5));
            stream.write("Hello World!".as_bytes()).unwrap();
            stream.flush().unwrap();
        });
    }
}
