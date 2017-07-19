use std::ffi::CString;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::net::TcpStream;

use super::Packet;
use super::PacketType::{SERVERDATA_AUTH, SERVERDATA_EXECCOMMAND};
use super::PacketDirection::{INCOMING, OUTCOMING};

const INVALID_RCON_ID: i32 = -1;

#[derive(Debug)]
pub struct Connection {
    packet_id: i32,
    connection: TcpStream
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

        // Somehow srcds returns us SERVERDATA_RESPONSE_VALUE first
        // we probably can safely ignore it
        let _ = Packet::read_from(&mut stream, INCOMING)?;

        let auth_packet_response = Packet::read_from(&mut stream, INCOMING)?;
        if auth_packet_response.id == INVALID_RCON_ID {
            Err(Error::new(
                ErrorKind::PermissionDenied,
                "invalid rcon password",
            ))
        } else {
            Ok(Connection {
                packet_id: 1,
                connection: stream
            })
        }
    }

    pub fn send_command(&mut self, command: &str) -> Result<CString, Error> {
        let cmd = CString::new(command)?;
        let command_packet = Packet {
            id: 1,
            net_type: SERVERDATA_EXECCOMMAND,
            body: cmd
        };

        self.connection.write(&command_packet.serialize())?;
        let response_body = Packet::read_from(&mut self.connection, INCOMING)?.body;
        Ok(response_body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::net::TcpListener;
    use std::error::Error;

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

            // empty auth packet
            let empty_response_packet = Packet {
                id: INVALID_RCON_ID,
                net_type: SERVERDATA_AUTH_RESPONSE,
                body: CString::new("").unwrap(),
            };

            let failed_rcon_packet = Packet {
                id: INVALID_RCON_ID,
                net_type: SERVERDATA_AUTH_RESPONSE,
                body: CString::new("").unwrap(),
            };

            let mut combined_packets = empty_response_packet.serialize();
            combined_packets.extend(failed_rcon_packet.serialize());
            stream.write(&combined_packets).unwrap();
        });

        match Connection::new(&hostname, INVALID_RCON_PASSWORD) {
            Ok(_) => panic!(),
            Err(e) => {
                assert_eq!(e.description(), "invalid rcon password");
                assert_eq!(e.kind(), ErrorKind::PermissionDenied);
            }
        }
    }

    #[test]
    fn test_send_command_success() {
        const RCON_COMMAND: &str = "echo 123";
        let (listener, hostname) = local_tcp_server();

        let t = thread::spawn(move || {
            let mut connection = Connection::new(&hostname, VALID_RCON_PASSWORD).unwrap();
            match connection.send_command(RCON_COMMAND) {
                Err(_) => panic!(),
                Ok(s) => assert_eq!("123", s.to_str().unwrap())
            }
        });

        let (mut stream, _) = listener.accept().unwrap();

        // empty auth packet
        let empty_response_packet = Packet {
            id: INVALID_RCON_ID,
            net_type: SERVERDATA_AUTH_RESPONSE,
            body: CString::new("").unwrap(),
        };

        // Send success to connection
        let success_auth_packet = Packet {
            id: 0,
            net_type: SERVERDATA_AUTH_RESPONSE,
            body: CString::new("").unwrap(),
        };

        let mut combined_packets = empty_response_packet.serialize();
        combined_packets.extend(success_auth_packet.serialize());
        stream.write(&combined_packets).unwrap();

        // Drop auth packet
        Packet::read_from(&mut stream, OUTCOMING).unwrap();

        // Check if incoming command packet is correct
        let command_packet = Packet::read_from(&mut stream, OUTCOMING).unwrap();

        assert_eq!(command_packet.id, 1);
        assert_eq!(command_packet.net_type, SERVERDATA_EXECCOMMAND);
        assert_eq!(
            command_packet.body.to_str().unwrap(),
            RCON_COMMAND
        );

        // and send command execution response
        let success_auth_packet = Packet {
            id: 3,
            net_type: SERVERDATA_AUTH_RESPONSE,
            body: CString::new("123").unwrap(),
        };
        stream.write(&success_auth_packet.serialize()).unwrap();

        t.join().unwrap();
    }

    #[test]
    fn test_send_command_fail_server_closed_connection() {
        const RCON_COMMAND: &str = "echo 123";
        let (listener, hostname) = local_tcp_server();

        let _t = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();

            // empty auth packet
            let empty_response_packet = Packet {
                id: INVALID_RCON_ID,
                net_type: SERVERDATA_AUTH_RESPONSE,
                body: CString::new("").unwrap(),
            };

            // Send success to connection
            let success_auth_packet = Packet {
                id: 0,
                net_type: SERVERDATA_AUTH_RESPONSE,
                body: CString::new("").unwrap(),
            };

            let mut combined_packets = empty_response_packet.serialize();
            combined_packets.extend(success_auth_packet.serialize());
            stream.write(&combined_packets).unwrap();
        });

        let mut connection = Connection::new(&hostname, VALID_RCON_PASSWORD).unwrap();
        if let Ok(_) = connection.send_command(RCON_COMMAND) {
            panic!()
        }
    }
}
