use std::io::{Error, ErrorKind};

use tokio::prelude::*;
use tokio_dns::TcpStream;
use tokio::net::TcpStream as TokioTcpStream;
use tokio_io::codec::Framed;

use srcds::rcon::async::codec::Codec as RconCodec;
use srcds::rcon::PacketType::*;
use srcds::rcon::Packet;

pub struct Connection {
    proto: Framed<TokioTcpStream, RconCodec>
}

const INVALID_RCON_ID: i32 = -1;

impl Connection {
    pub fn connect(host: &str, rcon_password: &str) -> Box<Future<Item = Connection, Error = Error> + Send> {
        let auth_packet = Packet::new(0, SERVERDATA_AUTH, rcon_password).unwrap();

        Box::new(
            TcpStream::connect(&host[..])
                .and_then(move |tcp| {
                    let proto = tcp.framed(RconCodec::new());
                    proto.send(auth_packet)
                })
                .and_then(|proto| proto.into_future().map_err(|(e, _)| e) )
                // skip SERVERDATA_RESPONSE_VALUE
                // TODO: Handle stream is over
                .and_then(|(_, proto)| proto.into_future().map_err(|(e, _)| e))
                .and_then(|(auth_response, proto)| {
                    if let Some(auth_response_packet) = auth_response {
                        if auth_response_packet.id == INVALID_RCON_ID {
                            Err(Error::new(ErrorKind::Other, "Invalid RCON password"))
                        } else {
                            Ok(Connection { proto: proto })
                        }
                    } else {
                        Err(Error::new(ErrorKind::Other, "Connection closed (?)"))
                    }
                }),
        )
    }
}
