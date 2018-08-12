#![cfg_attr(feature = "strict", deny(warnings))]

extern crate clap;
extern crate tokio;
extern crate futures;
extern crate rconcmd;

use std::io::{Error, ErrorKind};

use clap::{Arg, App};
use futures::{Future, Sink, Stream};

use rconcmd::srcds::rcon::AsyncConnection;
use rconcmd::srcds::rcon::PacketType::*;
use rconcmd::srcds::rcon::Packet;

fn main() {
    let matches = App::new("rconcmd")
        .version("0.2.0")
        .author("Fedcomp")
        .about("Rcon console for srcds servers")
        .arg(Arg::with_name("hostname").required(true).takes_value(true))
        .arg(
            Arg::with_name("rcon")
                .help("rcon_password of the server")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let hostname = matches.value_of("hostname").unwrap(); // unwrap because required
    let rcon_password = matches.value_of("rcon").unwrap(); // unwrap because required

    let connection = AsyncConnection::connect(hostname, rcon_password).and_then(|connection| {
        let proto = connection.proto;
        proto.send(Packet::new(0, SERVERDATA_EXECCOMMAND, "echo 123").unwrap())
    }).and_then(|proto| {
        let (proto_sink, proto_stream) = proto.split();

        // tokio::spawn(proto_stream.for_each(|packet| {
        //     if packet.net_type == SERVERDATA_RESPONSE_VALUE {
        //         println!("{}", packet.body.into_string().unwrap());
        //     }
        //
        //     Ok(())
        // }).map_err(|_| {
        //
        // }));

        // let stdin = spawn_stdin_stream_unbounded()
        // .for_each(move |line| {
        //     let command_packet = Packet::new(0, SERVERDATA_EXECCOMMAND, &line).unwrap();
        //     let sending_future = proto_sink.send(command_packet).and_then(|_| Ok(())).map_err(|_| {});
        //     // tokio::spawn(sending_future);
        //     Ok(())
        // });

        Ok(())
    }).map_err(|err| {
        println!("err = {:?}", err);
    });

    tokio::run(connection);
}
