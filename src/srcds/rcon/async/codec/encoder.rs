use std::io;

use bytes::BytesMut;
use tokio_io::codec::Encoder;

use srcds::rcon::Packet;
use super::Codec;

impl Encoder for Codec {
    type Item = Packet;
    type Error = io::Error;

    fn encode(&mut self, packet: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        println!("> {:?}", packet);
        let packet_contents = packet.serialize();
        buf.extend(packet_contents);
        Ok(())
    }
}

// TODO: Tests
