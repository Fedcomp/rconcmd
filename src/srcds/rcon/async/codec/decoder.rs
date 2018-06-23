use std::io;

use bytes::BytesMut;
use tokio_io::codec::Decoder;

use srcds::rcon::Packet;
use srcds::rcon::PacketDirection::INCOMING;

use super::Codec;

// TODO: Optimize buffer for read
fn read_packet(buf: &mut BytesMut) -> io::Result<Packet> {
    let (packet, size) = {
        let mut cursor = io::Cursor::new(&buf);
        (Packet::read_from(&mut cursor, INCOMING)?, cursor.position())
    };

    buf.split_to(size as usize);
    Ok(packet)
}

impl Decoder for Codec {
    type Item = Packet;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        match read_packet(buf) {
            Ok(packet) => {
                println!("< {:?}", packet);
                Ok(Some(packet))
            }
            Err(_) => Ok(None),
        }
    }
}

// TODO: Tests
