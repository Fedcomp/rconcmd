extern crate clap;
extern crate rconcmd;
extern crate tokio;
extern crate tokio_dns;
extern crate futures;
#[macro_use] extern crate failure;

use std::net::SocketAddr;
use std::io::Cursor;

use futures::future;
use clap::{Arg, App};
use tokio::io;
use tokio_dns::TcpStream;
use tokio::prelude::*;

use rconcmd::srcds::rcon::Packet;
use rconcmd::srcds::rcon::connection::INVALID_RCON_ID;
use rconcmd::srcds::rcon::PacketType::*;
use rconcmd::srcds::rcon::PacketDirection::*;
use rconcmd::srcds::rcon_async::codec::Codec as RconCodec;

fn main() {
    let matches = App::new("async_rconcmd")
                          .version("0.2.0")
                          .author("Fedcomp")
                          .about("Asynchronous rcon console for srcds servers")
                          .arg(Arg::with_name("hostname")
                               .required(true)
                               .takes_value(true))
                          .arg(Arg::with_name("rcon")
                               .help("rcon_password of the server")
                               .required(true)
                               .takes_value(true))
                          .get_matches();

    let hostname = matches.value_of("hostname").unwrap();
    let rcon_password = matches.value_of("rcon").unwrap().to_string();

    let connection = TcpStream::connect(hostname).and_then(|tcp| {
        println!("Connected");
        let proto = tcp.framed(RconCodec);
        let auth_packet = Packet::new(0, SERVERDATA_AUTH, "12345").unwrap();
        proto.send(auth_packet)
    }).and_then(|proto| {
        println!("Now listen {:?}", proto);
        proto.for_each(|stuff| {
            println!("Incoming {:?}", stuff);
            Ok(())
        })
    }).map_err(|err| {
        println!("err = {:?}", err);
    });

    tokio::run(connection);
}
