use std::io;

use tokio_dns::TcpStream;
use tokio::prelude::*;

use srcds::rcon::async::codec::Codec as RconCodec;
use srcds::rcon::PacketType::*;
use srcds::rcon::Packet;

pub struct Connection;

impl Connection {
    pub fn connect(host: &str, rcon_password: &str) -> Box<Future<Item = (), Error = io::Error> + Send> {
        let auth_packet = Packet::new(0, SERVERDATA_AUTH, rcon_password).unwrap();

        Box::new(
            TcpStream::connect(&host[..])
                .and_then(move |tcp| {
                    let proto = tcp.framed(RconCodec::new());
                    proto.send(auth_packet)
                })
                .and_then(|proto| proto.for_each(|_| {
                    Ok(())
                })),
        )
    }
}
