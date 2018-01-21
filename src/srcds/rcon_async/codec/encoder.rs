use std::io;
use bytes::BytesMut;
use tokio_io::codec::Encoder;
use super::super::super::rcon::Packet;
use super::Codec;

impl Encoder for Codec {
    type Item = Packet;
    type Error = io::Error;

    fn encode(&mut self, packet: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(packet.serialize());
        Ok(())
    }
}
