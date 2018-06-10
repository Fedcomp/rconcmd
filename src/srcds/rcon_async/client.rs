extern crate futures;
extern crate tokio_core;

use std::thread;

// use tokio_core::io::Io;
use tokio_core::reactor::Core;
use tokio_core::net::{TcpStream, TcpStreamNew};
use futures::{Stream, Sink, Future};
use futures::sync::mpsc;
use tokio_io::codec::Framed;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder};

use super::Codec;
use super::super::rcon::Packet;
use super::super::rcon::PacketType::SERVERDATA_AUTH;

use std::net::SocketAddr;
use std::ffi::CString;
use std::io;

use futures::future;

type RconFuture = Box<Future<Item = Packet, Error = io::Error>>;
type JustFuture = Box<Future<Item = (), Error = io::Error>>;

pub struct Client {
    addr: SocketAddr
}

impl Client {
    // https://github.com/jgallagher/tokio-chat-example/blob/master/tokio-chat-client/src/main.rs
    // Create the event loop and initiate the connection to the remote server
    pub fn connect(addr: SocketAddr, rcon: &str) -> io::Result<()> {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let tcp = TcpStream::connect(&addr, &handle);

        // tcp.and_then(Client::handshake);
        let handshake = tcp.and_then(|stream| {
            println!("Piping packet sending");
            let rcon_io = stream.framed(Codec);
            let packet = Packet::new(0, SERVERDATA_AUTH, rcon).unwrap();

            // rcon_io.send(packet).map(|handshake_io|{
            //     println!("handshake sent");
            //     println!("{:?}", handshake_io);
            //     // handshake_io.into_inner()
            //     handshake_io
            // })
            rcon_io.send(packet)
        }).and_then(|mut rcon_framed| {
            println!("handshake sent");
            println!("{:?}", rcon_framed);
            rcon_framed.poll().unwrap()
        })/*.and_then(|resp| {
            println!("{:?}", resp)
            future::ok(resp)
        })*/;

        core.run(handshake)?;
        Ok(())
    }
}
