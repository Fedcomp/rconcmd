#![cfg_attr(feature = "strict", deny(warnings))]

extern crate clap;
extern crate tokio;
extern crate futures;
extern crate rconcmd;
extern crate tokio_io;
extern crate tokio_game_protocols;

mod utils;

use utils::spawn_stdin_stream_unbounded;

use std::ffi::CString;

use clap::{Arg, App};
use futures::{Future, Sink, Stream};

use rconcmd::srcds::rcon::Connection;
use tokio_game_protocols::srcds::rcon::{Packet, PacketType::*};

use futures::sync::mpsc;

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

    let connection = Connection::connect(hostname, rcon_password)
    .and_then(|connection| {
        let proto = connection.proto;
        let packet_body = CString::new("echo 123").unwrap();
        let packet = Packet::new(0, SERVERDATA_EXECCOMMAND, packet_body);

        proto.send(packet)
    }).and_then(|proto| {
        let (proto_sink, proto_stream) = proto.split();
        let proto_sink = proto_sink.sink_map_err(|e| {
            println!("Sink error: {:?}", e);
        });

        let (tx, rx) = mpsc::unbounded();
        tokio::spawn(
            rx.forward(proto_sink).and_then(|_| Ok(())).map_err(|e| {
                println!("rx forward error: {:?}", e);
            })
        );

        // Stdin processing
        tokio::spawn(
            spawn_stdin_stream_unbounded().for_each(move |input| {
                let packet_body = CString::new(input).unwrap();
                let packet = Packet::new(0, SERVERDATA_EXECCOMMAND, packet_body);

                tx.unbounded_send(packet).and_then(|_| Ok(())).map_err(|e| {
                    println!("mspc send error: {:?}", e);
                })
            })
        );

        // Every incoming message
        proto_stream.for_each(|incoming_packet| {
            if incoming_packet.net_type != SERVERDATA_RESPONSE_VALUE {
                return Ok(())
            }

            match incoming_packet.body.into_string() {
                Ok(s) => print!("{}", s),
                Err(e) => print!("{:?}", e.into_cstring())
            };

            Ok(())
        })
    }).map_err(|err| {
        println!("err = {:?}", err);
    });

    tokio::run(connection);
}
