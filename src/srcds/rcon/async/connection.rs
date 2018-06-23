use std::net::SocketAddrV4;
use std::net::ToSocketAddrs;

use tokio_dns::TcpStream;
use tokio::prelude::*;

use srcds::rcon::async::codec::Codec as RconCodec;
use srcds::rcon::PacketType::*;
use srcds::rcon::Packet;

pub struct Connection;

impl Connection {
    pub fn connect(host: String) -> Box<Future<Item=(), Error=()> + Send> {
        Box::new(TcpStream::connect(&host[..]).and_then(|tcp| {
            let proto = tcp.framed(RconCodec);
            let auth_packet = Packet::new(0, SERVERDATA_AUTH, "12345").unwrap();
            proto.send(auth_packet)
        }).and_then(|proto| {
            proto.for_each(|stuff| {
                Ok(())
            })
        }).and_then(|_| {
            Ok(())
        }).map_err(|err| {
            println!("err = {:?}", err);
        }))
    }
}
