use std::io::ErrorKind::InvalidData;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::RwLock;
use std::time::Duration;

use serde::Serialize;

/// Describes the status of a minecraft server.
#[derive(Debug, Clone, Serialize)]
pub struct Status {
    addr: String,
    version: String,
    description: String,
    players: usize,
    max_players: usize,
}

impl Status {
    /// Performs server ping requests and updates the cache.
    pub async fn refresh<S: AsRef<str>>(state: &RwLock<Vec<Status>>, addresses: &[S]) {
        let mut current_status = Vec::with_capacity(addresses.len());
        for addr in addresses {
            current_status.push(
                Status::request(addr.as_ref())
                    .await
                    .unwrap_or_else(|_| Status::default(addr.as_ref())),
            );
        }
        let mut status = state.write().unwrap();
        *status = current_status;
    }

    /// Performes a classic server status request.
    ///
    /// The request looks as follows:
    /// - FE: packet identifier for a server list ping
    /// - 01: server list ping's payload (always 1)
    /// - ... optional data
    pub async fn request(addr: &str) -> io::Result<Status> {
        let socket_addr = ToSocketAddrs::to_socket_addrs(&addr)?
            .next()
            .ok_or(io::ErrorKind::AddrNotAvailable)?;

        let response = {
            let mut stream = TcpStream::connect_timeout(&socket_addr, Duration::from_millis(100))?;
            stream.write_all(&[0xfe, 0x01])?;
            let mut response = [0; 256];
            let _ = stream.read(&mut response[..])?;
            response
        };
        Status::parse(addr, &response)
    }

    /// Parses the classic server status request.
    ///
    /// The structure of this format is described below:
    /// After the first 3 bytes, the packet is a UTF-16BE string.
    /// It begins with two characters: §1, followed by a null character.
    /// On the wire these look like 00 a7 00 31 00 00.
    ///
    /// The remainder is null character (that is 00 00) delimited fields:
    /// - Protocol version (e.g. 74)
    /// - Minecraft server version (e.g. 1.8.7)
    /// - Message of the day (e.g. A Minecraft Server)
    /// - Current player count
    /// - Max players
    ///
    /// see: https://wiki.vg/Server_List_Ping#Client_to_server
    fn parse(addr: &str, response: &[u8]) -> io::Result<Status> {
        if response.len() <= 3 + 6 || response[0] != 0xff {
            return Err(InvalidData.into());
        }

        let length = u16::from_be_bytes([response[1], response[2]]) as usize;
        let response = &response[3..];

        if response.len() < 2 * length {
            return Err(InvalidData.into());
        }

        let response = response[..2 * length]
            .chunks_exact(2)
            .map(|e| u16::from_be_bytes([e[0], e[1]]))
            .collect::<Vec<_>>();
        let data = String::from_utf16_lossy(&response);

        let mut parts = data.split('\0');

        let _ident = parts.next().ok_or(InvalidData)?;
        let _protocol = parts.next().ok_or(InvalidData)?;

        let version = parts.next().ok_or(InvalidData)?.into();
        let description = parts.next().ok_or(InvalidData)?.into();
        let players = parts
            .next()
            .ok_or(InvalidData)?
            .parse()
            .map_err(|_| InvalidData)?;
        let max_players = parts
            .next()
            .ok_or(InvalidData)?
            .parse()
            .map_err(|_| InvalidData)?;

        let addr = addr.strip_suffix(":25565").unwrap_or(addr).into();

        Ok(Status {
            addr,
            version,
            description,
            players,
            max_players,
        })
    }

    /// Default status when a server is offline.
    fn default(addr: &str) -> Status {
        Status {
            addr: addr.strip_suffix(":25565").unwrap_or(addr).into(),
            version: String::new(),
            description: String::from("offline"),
            players: 0,
            max_players: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_request() {
        // normal
        let response = [
            0xff, 0x00, 0x23, 0x00, 0xa7, 0x00, 0x31, 0x00, 0x00, 0x00, 0x34, 0x00, 0x37, 0x00,
            0x00, 0x00, 0x31, 0x00, 0x2e, 0x00, 0x34, 0x00, 0x2e, 0x00, 0x32, 0x00, 0x00, 0x00,
            0x41, 0x00, 0x20, 0x00, 0x4d, 0x00, 0x69, 0x00, 0x6e, 0x00, 0x65, 0x00, 0x63, 0x00,
            0x72, 0x00, 0x61, 0x00, 0x66, 0x00, 0x74, 0x00, 0x20, 0x00, 0x53, 0x00, 0x65, 0x00,
            0x72, 0x00, 0x76, 0x00, 0x65, 0x00, 0x72, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00,
            0x32, 0x00, 0x30,
        ];
        let status = Status::parse("test.com", &response).expect("Parser error!");
        assert_eq!(status.addr, "test.com");
        assert_eq!(status.version, "1.4.2");
        assert_eq!(status.description, "A Minecraft Server");
        assert_eq!(status.players, 0);
        assert_eq!(status.max_players, 20);

        // padding
        let response = [
            0xff, 0x00, 0x23, 0x00, 0xa7, 0x00, 0x31, 0x00, 0x00, 0x00, 0x34, 0x00, 0x37, 0x00,
            0x00, 0x00, 0x31, 0x00, 0x2e, 0x00, 0x34, 0x00, 0x2e, 0x00, 0x32, 0x00, 0x00, 0x00,
            0x41, 0x00, 0x20, 0x00, 0x4d, 0x00, 0x69, 0x00, 0x6e, 0x00, 0x65, 0x00, 0x63, 0x00,
            0x72, 0x00, 0x61, 0x00, 0x66, 0x00, 0x74, 0x00, 0x20, 0x00, 0x53, 0x00, 0x65, 0x00,
            0x72, 0x00, 0x76, 0x00, 0x65, 0x00, 0x72, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00,
            0x32, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let status = Status::parse("test.com", &response).expect("Parser error!");
        assert_eq!(status.addr, "test.com");
        assert_eq!(status.version, "1.4.2");
        assert_eq!(status.description, "A Minecraft Server");
        assert_eq!(status.players, 0);
        assert_eq!(status.max_players, 20);

        // only prefix
        let response = [0xff, 0x00, 0x23];
        let status = Status::parse("test.com", &response);
        assert!(status.is_err());

        let response = [0xff, 0x00, 0x23, 0x00];
        let status = Status::parse("test.com", &response);
        assert!(status.is_err());
    }
}
