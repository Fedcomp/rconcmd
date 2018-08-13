#![allow(deprecated)]

use std::io::{Error, ErrorKind};
use std::ffi::CString;

use tokio::prelude::*;
use tokio_dns::TcpStream;
use tokio::net::TcpStream as TokioTcpStream;
use tokio_io::codec::Framed;

use tokio_game_protocols::srcds::rcon::{Codec, Packet, PacketType::*};

pub struct Connection {
    pub proto: Framed<TokioTcpStream, Codec>
}

const INVALID_RCON_ID: i32 = -1;

impl Connection {
    pub fn connect(host: &str, rcon_password: &str) -> impl Future<Item = Connection, Error = Error> {
        // TODO: Error handling
        let rcon_password = CString::new(rcon_password).unwrap();
        let auth_packet = Packet::new(0, SERVERDATA_AUTH, rcon_password);

        TcpStream::connect(&host[..])
            .and_then(move |tcp| {
                trace!("Connected {:?}", tcp);
                let proto = tcp.framed(Codec::default());
                proto.send(auth_packet)
            })
            // Generate request for first response packet
            .and_then(|proto| proto.into_future().map_err(|(e, _)| e) )
            // and skip SERVERDATA_RESPONSE_VALUE
            .map(|(_, proto)| proto )
            // TODO: Handle stream is over
            // Generate request for response auth
            .and_then(|proto| proto.into_future().map_err(|(e, _)| e) )
            .and_then(|(auth_response, proto)| {
                match auth_response {
                    Some(auth_response_packet) => {
                        if auth_response_packet.id == INVALID_RCON_ID {
                            Err(Error::new(ErrorKind::Other, "Invalid RCON password"))
                        } else {
                            Ok(Connection { proto: proto })
                        }
                    },
                    None => Err(Error::new(ErrorKind::Other, "Connection closed."))
                }
            })
    }
}
