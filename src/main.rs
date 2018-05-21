#[macro_use]
extern crate clap;
extern crate futures;

use futures::Future;

use std::net::*;
use std::time::{Duration};

enum PortStatus {
    Open,
    Closed
}

impl PortStatus {
    fn is_open(&self) -> bool {
        match self {
            &PortStatus::Open => true,
            &PortStatus::Closed => false,
        }
    }

    fn is_closed(&self) -> bool {
        !self.is_open()
    }
}

fn test_port(addr: &SocketAddr, timeout: Duration) -> PortStatus
{
    match TcpStream::connect_timeout(addr, timeout) {
        Ok(stream) => {
            stream.shutdown(Shutdown::Both).unwrap();
            PortStatus::Open
        },
        Err(_) => PortStatus::Closed
    }
}

fn print_status(host: &str, port: u16, status: &PortStatus) {
    let status = match status {
        &PortStatus::Closed => "CLOSED",
        &PortStatus::Open => "OPEN",
    };
    println!("{}:{:<6} | {}", host, port, status);
}

fn main() {
    let args = clap_app!(rascan =>
        (version: "0.1")
        (author: "Kaden Thomas")
        (about: "Scan TCP/UDP ports")
        (@arg HOST: +required "Host to scan")
        (@arg timeout: -t --timeout +takes_value "Set port timeout (in ms)")
        (@arg only_open: -o --open "Only show open ports")
    ).get_matches();

    let host = args.value_of("HOST")
        .expect("needed host argument"); 

    let timeout = args.value_of("timeout")
        .and_then(|t| t.parse::<u64>().ok())
        .unwrap_or(500);

    let timeout = Duration::from_millis(timeout);

    let only_open = args.is_present("only_open");

    for port in 0u16..1024 {
        let addrs: Vec<_> = (host, port).to_socket_addrs()
            .expect("unable to form host address")
            .collect();
        let addr = addrs.get(0)
            .expect("no valid host addresses");
        let status = test_port(&addr, timeout);
        if status.is_closed() && only_open {
            continue;
        } else {
            print_status(host, port, &status);
        }
    }
}
