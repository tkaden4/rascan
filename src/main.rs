#[macro_use]
extern crate clap;

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

fn scan_tcp(addr: &SocketAddr, timeout: Duration) -> PortStatus
{
    match TcpStream::connect_timeout(addr, timeout) {
        Ok(stream) => {
            stream.shutdown(Shutdown::Both).unwrap();
            PortStatus::Open
        },
        Err(_) => PortStatus::Closed
    }
}

struct Ports {
    current: u16,
    max: u16,
    host: IpAddr,
    timeout: Duration,
    done: bool
}

impl Ports {
    fn new(from: u16, to: u16, host: IpAddr, timeout: Duration) -> Self {
        Ports {
            current: from,
            max: to,
            host: host,
            timeout: timeout,
            done: false
        }
    }
}

impl Iterator for Ports {
    type Item = (u16, PortStatus);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.current == self.max {
            self.done = true;
        }
        let res = (self.host, self.current).to_socket_addrs().ok()
            .and_then(|mut x| x.nth(0))
            .map(|x| (self.current, scan_tcp(&x, self.timeout)));
        self.current += 1;
        res
    }
}

fn open_ports(host: IpAddr, timeout: Duration) -> Ports {
    Ports::new(0, 1024, host, timeout)
}

fn print_status(host: &str, port: u16, status: &PortStatus) {
    let status = match status {
        &PortStatus::Closed => "CLOSED",
        &PortStatus::Open => "OPEN",
    };
    println!("{}:{:<6} | {}", host, port, status);
}

fn resolve_host(host: &str) -> Option<IpAddr> {
    (host, 0).to_socket_addrs()
        .ok()
        .map(|x| x.map(|addr| addr.ip()))
        .and_then(|mut x| x.nth(0))
}

fn main() {
    let args = clap_app!(rascan =>
        (version: "0.1")
        (author: "Kaden Thomas")
        (about: "Scan TCP ports")
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

    let host_addr = resolve_host(host)
        .expect("unable to resolve host");

    open_ports(host_addr, timeout)
        .filter(|&(_, ref status)|{
            if only_open {
                status.is_open()
            } else {
                true
            }
        })
        .for_each(|(port, status)|{
            print_status(host, port, &status);
        });
}
