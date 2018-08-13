#![cfg_attr(feature = "strict", deny(warnings))]

extern crate clap;
extern crate tokio;
extern crate futures;
extern crate tokio_io;
extern crate tokio_timer;
extern crate tokio_game_protocols;
#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate lazy_static;
extern crate rconcmd;

use std::ffi::CString;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicBool};

use clap::{Arg, App};
use futures::{Future, Sink, Stream};
use futures::sync::mpsc;
use tokio_game_protocols::srcds::rcon::{Packet, PacketType::*};
use tokio_timer::Delay;

use rconcmd::srcds::rcon::Connection;
use rconcmd::utils::spawn_stdin_stream_unbounded;

lazy_static! {
    static ref PS_SCHEDULED: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

fn input_ps() {
    if PS_SCHEDULED.load(Ordering::Relaxed) == true {
        return;
    } else {
        let timer = Delay::new(Instant::now() + Duration::new(0, 10000))
            .and_then(|_| {
                print!("rcon> ");
                io::stdout().flush().unwrap();
                PS_SCHEDULED.store(false, Ordering::Relaxed);
                Ok(())
            }).map_err(|e| {
                error!("Timer error: {:?}", e);
            });

        PS_SCHEDULED.store(true, Ordering::Relaxed);
        tokio::spawn(timer);
    }
}

fn main() {
    env_logger::init();

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

    let hostname = matches.value_of("hostname").unwrap(); // required
    let rcon_password = matches.value_of("rcon").unwrap(); // required

    let connection = Connection::connect(hostname, rcon_password)
    .map(|connection| connection.proto )
    .and_then(|proto| {
        let (proto_sink, proto_stream) = proto.split();
        let proto_sink = proto_sink.sink_map_err(|e| {
            error!("Sink error: {:?}", e);
        });

        let (tx, rx) = mpsc::unbounded();
        tokio::spawn(
            rx.forward(proto_sink).map(|_| ()).map_err(|e| {
                error!("rx forward error: {:?}", e);
            })
        );

        input_ps();
        tokio::spawn(
            spawn_stdin_stream_unbounded().for_each(move |input| {
                if input == "" {
                    return Ok(());
                }

                let packet_body = CString::new(input).unwrap();
                let packet = Packet::new(0, SERVERDATA_EXECCOMMAND, packet_body);

                tx.unbounded_send(packet).and_then(|_| Ok(())).map_err(|e| {
                    error!("mspc send error: {:?}", e);
                })
            })
        );

        proto_stream.for_each(|incoming_packet| {
            if incoming_packet.net_type != SERVERDATA_RESPONSE_VALUE {
                return Ok(())
            }

            match incoming_packet.body.into_string() {
                Ok(s) => print!("{}", s),
                Err(e) => print!("{:?}", e.into_cstring())
            };

            input_ps();
            Ok(())
        })
    }).map_err(|err| {
        error!("err = {:?}", err);
    });

    tokio::run(connection)
}
