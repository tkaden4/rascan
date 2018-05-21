#[macro_use]
extern crate clap;
extern crate pnet;

use std::net::*;
use std::time::Duration;

mod ports;
use ports::*;

fn print_status(port: u16, status: &PortStatus) {
    let status = match status {
        &PortStatus::Closed => "CLOSED",
        &PortStatus::Open => "OPEN",
    };
    println!("{:<6} | {}", port, status);
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
        (@arg start: -s --start +takes_value "Start port")
        (@arg end: -e --end +takes_value "End port")
    ).get_matches();

    let start_port = args.value_of("start")
        .and_then(|x| x.parse::<u16>().ok())
        .unwrap_or(0);

    let end_port = args.value_of("end")
        .and_then(|x| x.parse::<u16>().ok())
        .unwrap_or(if args.is_present("start") { start_port } else { 1024 });
    
    let timeout_ms = args.value_of("timeout")
        .and_then(|t| t.parse::<u64>().ok())
        .unwrap_or(500);
    let timeout = Duration::from_millis(timeout_ms);

    let only_open = args.is_present("only_open");

    let host = args.value_of("HOST")
        .expect("needed host argument");
    let host_addr = resolve_host(host)
        .expect("unable to resolve host");

    ports(host_addr, timeout, start_port, end_port)
        .filter(|&(_, ref status)|{
            if only_open {
                status.is_open()
            } else {
                true
            }
        })
        .for_each(|(port, status)|{
            print_status(port, &status);
        });
}
