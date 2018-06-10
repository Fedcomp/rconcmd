use std::io;
use bytes::BytesMut;
use tokio_io::codec::Decoder;
use super::super::super::rcon::PacketDirection::INCOMING;
use super::super::super::rcon::Packet;

use super::Codec;

// TODO: Optimize buffer for read
fn read_packet(buf: &mut BytesMut) -> io::Result<Packet> {
    let mut stream = io::Cursor::new(buf);
    let packet = Packet::read_from(&mut stream, INCOMING)?;
    Ok(packet)
}

impl Decoder for Codec {
    type Item = Packet;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        match read_packet(buf) {
            Ok(packet) => {
                println!("> {:?}", packet);
                Ok(Some(packet))
            },
            Err(_) => Ok(None)
        }
    }
}
