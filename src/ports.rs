use std::net::*;
use std::time::Duration;

pub enum PortStatus {
    Open,
    Closed
}

impl PortStatus {
    pub fn is_open(&self) -> bool {
        match self {
            &PortStatus::Open => true,
            &PortStatus::Closed => false,
        }
    }

    pub fn is_closed(&self) -> bool {
        !self.is_open()
    }
}

pub fn scan_tcp(addr: &SocketAddr, timeout: Duration) -> Option<PortStatus>
{
    match TcpStream::connect_timeout(addr, timeout) {
        Ok(stream) => {
            stream.shutdown(Shutdown::Both).ok()?;
            Some(PortStatus::Open)
        },
        Err(_) => Some(PortStatus::Closed)
    }
}

pub struct Ports {
    current: u16,
    max: u16,
    host: IpAddr,
    timeout: Duration,
    done: bool
}

impl Ports {
    pub fn new(from: u16, to: u16, host: IpAddr, timeout: Duration) -> Self {
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
            .and_then(|x| scan_tcp(&x, self.timeout)
                      .map(|stat| (self.current, stat)));
        if !self.done {
            self.current += 1;
        }
        res
    }
}

pub fn ports(host: IpAddr, timeout: Duration, start: u16, end: u16) -> Ports {
    Ports::new(start, end, host, timeout)
}
