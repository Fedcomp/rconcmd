use std::ffi::CString;
use std::mem;
use std::io::Read;
use std::io::Error;
use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use bytes::{BytesMut, BufMut};

use super::packet_type::PacketType;
use super::packet_type::PacketDirection;

#[derive(Debug)]
pub struct Packet {
    pub id: i32, // 4 bytes, server can return -1 when rcon is invalid
    pub net_type: PacketType,
    pub body: CString,
}

impl Packet {
    fn size(&self) -> u32 {
        // const
        let u32_size: usize = mem::size_of::<u32>();

        // packet size is not used in size
        (
            u32_size + // packet id
            u32_size + // net_type
            self.body.as_bytes_with_nul().len() + // teminated body
            1 // empty terminated string
        ) as u32
    }

    pub fn serialize(&self) -> BytesMut {
        let packet_size = self.size();
        let mut buff = BytesMut::with_capacity(packet_size as usize);

        buff.put_u32::<LittleEndian>(packet_size);
        buff.put_i32::<LittleEndian>(self.id);
        buff.put_i32::<LittleEndian>(self.net_type.value());
        buff.extend(self.body.as_bytes());
        buff.put_slice(b"\x00\x00");
        buff
    }

    pub fn read_from<S: Read>(stream: &mut S, direction: PacketDirection) -> Result<Packet, Error> {
        // packet id (4) - packet size (4) - two zero bytes (2)
        const SERVICE_FIELDS_SIZE: usize = 10;

        let packet_size = stream.read_i32::<LittleEndian>()? as usize;
        let mut raw_packet: Vec<u8> = vec![0; packet_size];
        stream.read_exact(&mut raw_packet)?;

        let mut stream = Cursor::new(raw_packet);
        let id = stream.read_i32::<LittleEndian>()?;
        let net_type = stream.read_i32::<LittleEndian>()?;
        let net_type = PacketType::from_value(net_type, direction);
        let mut body = vec![0; packet_size - SERVICE_FIELDS_SIZE];
        stream.read_exact(&mut body)?;
        let body = CString::new(body)?;

        Ok(Packet {
            id: id,
            net_type: net_type,
            body: body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::packet_type::PacketDirection::OUTCOMING;

    const OUTGOING_AUTH_PACKET: [u8; 21] = [
        0x11, 0x00, 0x00, 0x00, // Size
        0x00, 0x00, 0x00, 0x00, // id
        0x03, 0x00, 0x00, 0x00, // type
        0x70, 0x61, 0x73, 0x73, 0x77, 0x72, 0x64, 0x00, // command string (passwd in this case)
        0x00 // packet temination string
    ];

    #[test]
    fn test_size() {
        let mut packet = Packet {
            id: 0,
            net_type: PacketType::SERVERDATA_AUTH,
            body: CString::new("").unwrap(),
        };

        assert_eq!(packet.size(), 10);

        packet.body = CString::new("body").unwrap();
        assert_eq!(packet.size(), 14);
    }

    #[test]
    fn test_serialize() {
        let packet = Packet {
            id: 0,
            net_type: PacketType::SERVERDATA_AUTH,
            body: CString::new("passwrd").unwrap(),
        };

        assert_eq!(packet.serialize()[..], OUTGOING_AUTH_PACKET);
    }

    #[test]
    fn test_read_from() {
        let mut stream = Cursor::new(OUTGOING_AUTH_PACKET);
        let packet = Packet::read_from(&mut stream, OUTCOMING).unwrap();

        assert_eq!(packet.id, 0);
        assert_eq!(packet.net_type, PacketType::SERVERDATA_AUTH);
        assert_eq!(packet.body, CString::new("passwrd").unwrap());
    }
}
