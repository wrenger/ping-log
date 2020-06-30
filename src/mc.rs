use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::RwLock;
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
    pub fn refresh<S: AsRef<str>>(state: &RwLock<Vec<Status>>, addresses: &[S]) {
        let mut current_status = Vec::with_capacity(addresses.len());
        for addr in addresses {
            current_status.push(
                Status::request(addr.as_ref()).unwrap_or_else(|_| Status::default(addr.as_ref())),
            );
        }
        let mut status = state.write().unwrap();
        *status = current_status;
    }

    pub fn request(addr: &str) -> io::Result<Status> {
        let socket_addr = ToSocketAddrs::to_socket_addrs(&addr)?
            .next()
            .ok_or(io::ErrorKind::AddrNotAvailable)?;

        let response = {
            let mut stream = TcpStream::connect_timeout(&socket_addr, Duration::from_millis(100))?;
            stream.write_all(&[0xFE, 0x01])?;
            let mut response = vec![0; 512];
            let _ = stream.read(&mut response[..])?;
            response
        };
        Status::parse(addr, &response)
    }

    fn parse(addr: &str, response: &[u8]) -> io::Result<Status> {
        let response = response
            .chunks_exact(2)
            .map(|e| u16::from_le_bytes([e[0], e[1]]))
            .collect::<Vec<_>>();
        let data = String::from_utf16_lossy(&response);
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

    fn default(addr: &str) -> Status {
        Status {
            addr: String::from(addr),
            version: String::new(),
            description: String::from("offline"),
            players: 0,
            max_players: 0,
        }
    }
}
