#![cfg_attr(feature = "strict", deny(warnings))]

extern crate clap;
extern crate rconcmd;
extern crate tokio;
extern crate tokio_dns;
extern crate futures;

use clap::{Arg, App};
use futures::{Future, Sink, Stream};

use rconcmd::srcds::rcon::AsyncConnection;
use rconcmd::srcds::rcon::PacketType::*;
use rconcmd::srcds::rcon::Packet;

fn main() {
    let matches = App::new("async_rconcmd")
        .version("0.2.0")
        .author("Fedcomp")
        .about("Asynchronous rcon console for srcds servers")
        .arg(Arg::with_name("hostname").required(true).takes_value(true))
        .arg(
            Arg::with_name("rcon")
                .help("rcon_password of the server")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let hostname = matches.value_of("hostname").unwrap();
    let rcon_password = matches.value_of("rcon").unwrap();
    let connection = AsyncConnection::connect(hostname, rcon_password).and_then(|connection| {
        let proto = connection.proto;
        proto.send(Packet::new(0, SERVERDATA_EXECCOMMAND, "echo 123").unwrap())
    }).and_then(|proto| {
        proto.for_each(|packet| {
            if packet.net_type == SERVERDATA_RESPONSE_VALUE {
                println!("{}", packet.body.into_string().unwrap());
            }

            Ok(())
        })
    }).map_err(|err| {
        println!("err = {:?}", err);
    });

    tokio::run(connection);
}
