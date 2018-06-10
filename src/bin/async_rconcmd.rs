extern crate clap;
extern crate tokio_core;
extern crate rconcmd;

use std::net::SocketAddr;
use clap::{Arg, App};
use rconcmd::srcds::rcon_async::ClientRconProto;
use rconcmd::srcds::rcon::Packet;
use rconcmd::srcds::rcon::PacketType::*;


extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use futures::future;
use futures::{Future, Stream, Sink};

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_core::reactor::Core;
use tokio_proto::{TcpClient, TcpServer};
use tokio_proto::pipeline::{ClientProto, ServerProto};
use tokio_service::{Service, NewService};

use std::{io, thread};
use std::time::Duration;

fn main() {
    let matches = App::new("rconcmd")
                          .version("0.2.0")
                          .author("Fedcomp")
                          .about("Rcon console for srcds servers")
                          .arg(Arg::with_name("hostname")
                               .required(true)
                               .takes_value(true))
                          .arg(Arg::with_name("rcon")
                               .required(true)
                               .help("rcon_password of the server")
                               .takes_value(true))
                          .arg(Arg::with_name("execute")
                               .short("e")
                               .long("execute")
                               .help("Execute single command, print whole response and quit")
                               .takes_value(true))
                          .get_matches();

    let hostname = matches.value_of("hostname").unwrap().parse::<SocketAddr>().unwrap();
    let rcon_password = matches.value_of("rcon").unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = TcpClient::new(ClientRconProto).connect(&hostname, &handle);

    let data = client.and_then(|client| {
        let echo_packet = Packet::new(0, SERVERDATA_EXECCOMMAND, "echo 123").unwrap();
        client.call(echo_packet)
    }).and_then(|resp| {
        println!("resp {:?}", resp);
        future::ok(resp)
    });

    core.run(data).unwrap();
    //
    // core.run(
    //     client
    //         .and_then(|client| {
    //             client.call("Goodbye".to_string())
    //                 .and_then(|response| {
    //                     println!("CLIENT: {:?}", response);
    //                     Ok(())
    //                 })
    //         })
    // ).unwrap();
}
