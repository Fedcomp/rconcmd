use std::ffi::CString;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use super::Packet;
use super::PacketType::SERVERDATA_AUTH;

#[derive(Debug)]
pub struct Connection {
    packet_id: i32
}

impl Connection {
    fn new(hostname: &str, rcon_password: &str) -> Result<Connection, io::Error> {
        let mut stream = TcpStream::connect(hostname).unwrap();

        let auth_packet = Packet {
            id: 0,
            net_type: SERVERDATA_AUTH,
            body: CString::new(rcon_password).unwrap()
        };

        stream.write(&auth_packet.serialize()).unwrap();

        Ok(Connection {
            packet_id: 0
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::net::TcpListener;

    const RCON_PASSWORD: &str = "somespecialrconpassword";

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
            let _connection = Connection::new(&hostname, RCON_PASSWORD).unwrap();
        });

        let (mut stream, _) = listener.accept().unwrap();
        let connection_packet = Packet::read_from(&mut stream).unwrap();

        assert_eq!(connection_packet.id, 0);
        assert_eq!(connection_packet.net_type, SERVERDATA_AUTH);
        assert_eq!(connection_packet.body.to_str().unwrap(), RCON_PASSWORD);
    }

    // #[test]
    // fn test_new_fail_unreachable() {
    //     let _connection_result = Connection::new("1.1.1.1:9999", RCON_PASSWORD);
    // }
}
