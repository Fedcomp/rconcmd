use std::io;
use bytes::BytesMut;
use tokio_io::codec::Decoder;
use super::super::super::rcon::PacketDirection::INCOMING;
use super::super::super::rcon::Packet;

use super::Codec;

// TODO: Optimize buffer for read
fn read_packet(buf: &mut BytesMut) -> io::Result<Packet> {
    let (packet, size) = {
        let mut cursor = io::Cursor::new(&buf);
        (Packet::read_from(&mut cursor, INCOMING)?, cursor.position())
    };

    buf.drain_to(size as usize);
    Ok(packet)
}

impl Decoder for Codec {
    type Item = Packet;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        // println!("Decode!");
        match read_packet(buf) {
            Ok(packet) => {
                println!("< {:?}", packet);
                Ok(Some(packet))
            },
            Err(_) => Ok(None)
        }
    }
}
