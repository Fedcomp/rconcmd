use std::ffi::CString;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::net::TcpStream;

use super::Packet;
use super::PacketType::SERVERDATA_AUTH;
use super::PacketDirection::OUTCOMING;

const INVALID_RCON_ID: i32 = -1;

#[derive(Debug)]
pub struct Connection {
    packet_id: i32,
}

impl Connection {
    pub fn new(hostname: &str, rcon_password: &str) -> Result<Connection, Error> {
        let mut stream = TcpStream::connect(hostname)?;

        let auth_packet = Packet {
            id: 0,
            net_type: SERVERDATA_AUTH,
            body: CString::new(rcon_password).unwrap(),
        };

        stream.write(&auth_packet.serialize())?;

        let auth_packet_response = Packet::read_from(&mut stream, OUTCOMING)?;
        if auth_packet_response.id == INVALID_RCON_ID {
            Err(Error::new(
                ErrorKind::PermissionDenied,
                "invalid rcon password",
            ))
        } else {
            Ok(Connection { packet_id: 1 })
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::net::TcpListener;
    use std::error::Error;

    use super::super::PacketDirection::INCOMING;
    use super::super::PacketType::SERVERDATA_AUTH_RESPONSE;

    const VALID_RCON_PASSWORD: &str = "somespecialrconpassword";
    const INVALID_RCON_PASSWORD: &str = "somenonspecialrconpassword";

    fn local_tcp_server() -> (TcpListener, String) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let listen_port = listener.local_addr().unwrap().port();
        let hostname = format!("127.0.0.1:{}", listen_port);

        (listener, hostname)
    }

    #[test]
    fn test_new_success() {
        let (listener, hostname) = local_tcp_server();

        let _t = thread::spawn(move || {
            let _ = Connection::new(&hostname, VALID_RCON_PASSWORD);
        });

        let (mut stream, _) = listener.accept().unwrap();
        let connection_packet = Packet::read_from(&mut stream, INCOMING).unwrap();

        assert_eq!(connection_packet.id, 0);
        assert_eq!(connection_packet.net_type, SERVERDATA_AUTH);
        assert_eq!(
            connection_packet.body.to_str().unwrap(),
            VALID_RCON_PASSWORD
        );
    }

    #[test]
    fn test_new_fail_rcon() {
        let (listener, hostname) = local_tcp_server();

        let _t = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();

            let failed_rcon_packet = Packet {
                id: INVALID_RCON_ID,
                net_type: SERVERDATA_AUTH_RESPONSE,
                body: CString::new("").unwrap(),
            };
            stream.write(&failed_rcon_packet.serialize()).unwrap();
        });

        match Connection::new(&hostname, INVALID_RCON_PASSWORD) {
            Ok(_) => panic!(),
            Err(e) => {
                assert_eq!(e.description(), "invalid rcon password");
                assert_eq!(e.kind(), ErrorKind::PermissionDenied);
            }
        }
    }
}
