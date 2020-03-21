use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::slice;
use std::time::Duration;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Status {
    addr: String,
    version: String,
    description: String,
    players: usize,
    max_players: usize,
}

impl Status {
    pub fn request(addr: &str) -> io::Result<Status> {
        let socket_addr = ToSocketAddrs::to_socket_addrs(&addr)?
            .next()
            .ok_or(io::ErrorKind::AddrNotAvailable)?;

        let response = {
            let mut stream = TcpStream::connect_timeout(&socket_addr, Duration::from_secs(1))?;
            stream.write(&[0xFE, 0x01])?;
            let mut response = vec![0; 512];
            stream.read(&mut response[..])?;
            response
        };
        Status::parse(addr, &response)
    }

    fn parse(addr: &str, response: &[u8]) -> io::Result<Status> {
        let response: &[u16] =
            unsafe { slice::from_raw_parts(response.as_ptr() as *const _, response.len() / 2) };
        let data = String::from_utf16_lossy(response);
        let data = data.splitn(7, '\0').collect::<Vec<_>>();
        if data.len() == 7 {
            Ok(Status {
                addr: addr.into(),
                version: data[2].into(),
                description: data[3].into(),
                players: data[4].parse().map_err(|_| io::ErrorKind::InvalidData)?,
                max_players: data[5].parse().map_err(|_| io::ErrorKind::InvalidData)?,
            })
        } else {
            Err(io::ErrorKind::InvalidData.into())
        }
    }

    pub fn default(addr: &str) -> Status {
        Status {
            addr: String::from(addr),
            version: String::new(),
            description: String::from("offline"),
            players: 0,
            max_players: 0,
        }
    }
}
