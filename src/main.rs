#[macro_use] extern crate clap;
#[macro_use] extern crate matches;
extern crate rayon;
extern crate nix;
extern crate ansi_term;

use rayon::prelude::*;

use std::net::*;
use std::time::Duration;

use clap::ArgMatches;

// TODO make cross-platform
use nix::unistd::isatty;

use ansi_term::*;

mod ports;

use ports::*;

fn paint_tty(text: &str, color: Colour) -> String {
    if isatty(1).expect("impossible") {
        color.paint(text).to_string()
    } else {
        text.into()
    }
}

fn print_status(port: u16, status: PortStatus) {
    let status = match status {
        PortStatus::Closed => paint_tty("CLOSED", Colour::Red),
        PortStatus::Open => paint_tty("OPEN", Colour::Green)
    };
    println!("{:<5} {}", port, status);
}

fn resolve_host(host: &str) -> Option<IpAddr> {
    (host, 0).to_socket_addrs().ok()
        .map(|x| x.map(|addr| addr.ip()))
        .and_then(|mut x| x.nth(0))
}

fn rascan(args: ArgMatches) -> Result<(), String> {
    let start_port = args.value_of("start")
        .unwrap()
        .parse::<u16>()
        .map_err(|_| "unable to parse start port".to_owned())?;

    let end_port = args.value_of("end")
        .unwrap()
        .parse::<u16>()
        .map_err(|_| "unable to parse end port".to_owned())?;

    let timeout_ms = args.value_of("timeout")
        .unwrap_or("500")
        .parse::<u64>()
        .map_err(|_| "unable to parse timeout")?;

    let timeout = Duration::from_millis(timeout_ms);

    let only_open = args.is_present("only_open");

    let host = args.value_of("HOST")
        .ok_or("need host argument".to_owned())?;

    let host_addr = resolve_host(host)
        .ok_or("unable to resolve host".to_owned())?;

    ports(host_addr, timeout, start_port, end_port)
        .filter(|&(_, ref status)|{
            if only_open {
                status.is_open()
            } else {
                true
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(port, status)|{
            print_status(port, status);
        });
    Ok(())
}

fn main() {
    let args = clap_app!(rascan =>
        (version: "0.1")
        (author: "Kaden Thomas")
        (about: "Scan TCP ports")
        (@arg HOST: +required "Host to scan")
        (@arg timeout: -t --timeout +takes_value "Set port timeout (in ms)")
        (@arg only_open: -o --open "Only show open ports")
        (@arg start: -s --start +required +takes_value "Start port")
        (@arg end: -e --end +required +takes_value "End port")
    ).get_matches();

    if let Err(err) = rascan(args) {
        eprintln!("rascan: {}", err);
    }
}
