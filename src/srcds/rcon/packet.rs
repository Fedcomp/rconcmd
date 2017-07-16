use std::ffi::CString;
use std::mem;

extern crate byteorder;
use self::byteorder::{LittleEndian, WriteBytesExt};

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
        buff.write_i32::<LittleEndian>(self.net_type as i32).unwrap();
        buff.extend(self.body.as_bytes());
        buff.push(0);
        buff.push(0);

        buff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        const EXPECTED_DATA: [u8; 21] = [
            0x11, 0x00, 0x00, 0x00, // Size
            0x00, 0x00, 0x00, 0x00, // id
            0x03, 0x00, 0x00, 0x00, // type
            0x70, 0x61, 0x73, 0x73, 0x77, 0x72, 0x64, 0x00, // command string (passwd in this case)
            0x00 // packet temination string
        ];

        assert_eq!(packet.serialize()[..], EXPECTED_DATA);
    }
}
