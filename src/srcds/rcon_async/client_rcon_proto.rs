use std::io;
use futures::{Future, Stream, Sink};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ClientProto;

use super::Codec;
use super::super::rcon::Packet;
use super::super::rcon::PacketType::*;

pub struct ClientRconProto;

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for ClientRconProto {
    type Request = Packet;
    type Response = Packet;

    /// `Framed<T, LineCodec>` is the return value of `io.framed(LineCodec)`
    type Transport = Framed<T, Codec>;
    type BindTransport = Box<Future<Item = Self::Transport, Error = io::Error>>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let transport = io.framed(Codec);

        println!("Binding transport");
        // Send the handshake frame to the server.
        let auth_packet = Packet::new(0, SERVERDATA_AUTH, "321312313").unwrap();
        let handshake = transport.send(auth_packet)
            // Wait for a response from the server, if the transport errors out,
            // we don't care about the transport handle anymore, just the error
            .and_then(|transport| {
                println!("Conversion");
                transport.into_future().map_err(|(e, _)| e)
            })
            .and_then(|(line, transport)| {
                // The server sent back a line, check to see if it is the
                // expected handshake line.
                Ok(transport)
                // match line {
                //     Some(ref msg) if msg == "Bring it!" => {
                //         println!("CLIENT: received server handshake");
                //         Ok(transport)
                //     }
                //     Some(ref msg) if msg == "No! Go away!" => {
                //         // At this point, the server is at capacity. There are a
                //         // few things that we could do. Set a backoff timer and
                //         // try again in a bit. Or we could try a different
                //         // remote server. However, we're just going to error out
                //         // the connection.
                //
                //         println!("CLIENT: server is at capacity");
                //         let err = io::Error::new(io::ErrorKind::Other, "server at capacity");
                //         Err(err)
                //     }
                //     _ => {
                //         println!("CLIENT: server handshake INVALID");
                //         let err = io::Error::new(io::ErrorKind::Other, "invalid handshake");
                //         Err(err)
                //     }
                // }
            });

        Box::new(handshake)
    }
}
