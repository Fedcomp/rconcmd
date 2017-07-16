use std::ffi::CString;
use std::mem;
use std::io::Read;
use std::io::Error;
use std::io::Cursor;

extern crate byteorder;
use self::byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};

use super::PacketType;

pub struct Packet {
    pub id: i32,
    pub net_type: PacketType,
    pub body: CString
}

impl Packet {
    fn size(&self) -> i32 {
        // packet size is not used in size
        // packet id + net_type + teminated body + empty terminated string
        (mem::size_of::<i32>() + mem::size_of::<i32>() + self.body.as_bytes_with_nul().len() + 1) as i32
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buff = vec![];

        buff.write_i32::<LittleEndian>(self.size()).unwrap();
        buff.write_i32::<LittleEndian>(self.id).unwrap();
        buff.write_i32::<LittleEndian>(self.net_type.value()).unwrap();
        buff.extend(self.body.as_bytes());
        buff.push(0);
        buff.push(0);

        buff
    }

    pub fn read_from<S>(stream: &mut S) -> Result<Packet, Error> where S: Read {
        let packet_size = stream.read_i32::<LittleEndian>().unwrap();
        let mut packet: Vec<u8> = vec![0; packet_size as usize];
        let _ = stream.read_exact(&mut packet);
        let mut stream = Cursor::new(packet);

        let id = stream.read_i32::<LittleEndian>().unwrap();
        let net_type = stream.read_i32::<LittleEndian>().unwrap();
        let net_type = PacketType::from_value(net_type, false);
        let mut body = vec![0; (packet_size as usize) - 4 - 4 - 2];
        let _ = stream.read_exact(&mut body);
        let body = CString::new(body).unwrap();

        Ok(Packet {
            id: id,
            net_type: net_type,
            body: body
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            body: CString::new("").unwrap()
        };

        assert_eq!(packet.size(), 10);

        packet.body = CString::new("body").unwrap();
        assert_eq!(packet.size(), 14);
        assert_eq!(packet.size(), (packet.serialize().len() - 4) as i32);
    }

    #[test]
    fn test_serialize() {
        let packet = Packet {
            id: 0,
            net_type: PacketType::SERVERDATA_AUTH,
            body: CString::new("passwrd").unwrap()
        };

        assert_eq!(packet.serialize()[..], OUTGOING_AUTH_PACKET);
    }

    #[test]
    fn test_read_from() {
        let mut stream = Cursor::new(OUTGOING_AUTH_PACKET);
        let packet = Packet::read_from(&mut stream).unwrap();

        assert_eq!(packet.id, 0);
        assert_eq!(packet.net_type, PacketType::SERVERDATA_AUTH);
        assert_eq!(packet.body, CString::new("passwrd").unwrap());
    }
}
