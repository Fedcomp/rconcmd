use std::ffi::CString;
use std::mem;

extern crate byteorder;
use self::byteorder::{LittleEndian, WriteBytesExt};

#[allow(non_camel_case_types)]
#[derive(Clone)]
#[derive(Copy)]
pub enum PacketType {
    SERVERDATA_AUTH = 3,
    // SERVERDATA_AUTH_RESPONSE = 2,
    SERVERDATA_EXECCOMMAND = 2,
    SERVERDATA_RESPONSE_VALUE = 0
}

pub struct Packet {
    id: i32,
    net_type: PacketType,
    body: CString
}

impl Packet {
    fn size(&self) -> i32 {
        // packet size is not used in size
        // packet id + net_type + teminated body + empty terminated string
        (mem::size_of::<i32>() + mem::size_of::<i32>() + self.body.as_bytes_with_nul().len() + 1) as i32
    }

    fn serialize(&self) -> Vec<u8> {
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

        let mut new_body = CString::new("body").unwrap();
        packet.body = new_body;
        assert_eq!(packet.size(), 14);
    }

    #[test]
    fn test_serialize() {
        let mut packet = Packet {
            id: 0,
            net_type: PacketType::SERVERDATA_AUTH,
            body: CString::new("passwrd").unwrap()
        };

        const EXPECTED_DATA: [u8; 21] = [
            0x11, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x03, 0x00, 0x00, 0x00,
            0x70, 0x61, 0x73, 0x73, 0x77, 0x72, 0x64, 0x00,
            0x00
        ];

        assert_eq!(packet.serialize()[..], EXPECTED_DATA);
    }
}
