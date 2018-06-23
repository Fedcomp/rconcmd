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
use rconcmd::srcds::rcon::async::codec::Codec as RconCodec;
use rconcmd::srcds::rcon::AsyncConnection;

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

    let connection = AsyncConnection::connect(hostname.to_string());

    tokio::run(connection);
}
