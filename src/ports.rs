use std::net::*;
use std::time::Duration;

pub enum PortStatus {
    Open,
    Closed
}

use PortStatus::*;

impl PortStatus {
    pub fn is_open(&self) -> bool {
        matches!(*self, Open)
    }

    pub fn is_closed(&self) -> bool {
        !self.is_open()
    }
}

pub fn scan_tcp(addr: &SocketAddr, timeout: Duration) -> PortStatus
{
    match TcpStream::connect_timeout(addr, timeout) {
        Ok(stream) => {
            stream.shutdown(Shutdown::Both).unwrap();
            Open
        },
        Err(_) => Closed
    }
}

fn get_addr(host: IpAddr, port: u16) -> SocketAddr {
    (host, port).to_socket_addrs()
        .ok()
        .and_then(|mut x| x.nth(0))
        .unwrap()
}

pub fn ports(host: IpAddr, timeout: Duration, start: u16, end: u16) -> impl Iterator<Item = (u16, PortStatus)> {
    //Ports::new(start, end, host, timeout)
    (start..end)
        .map(move |x| (x, scan_tcp(&get_addr(host, x), timeout)))
}
