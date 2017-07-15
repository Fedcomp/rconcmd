use std::io::prelude::*;
use std::net::TcpStream;
use std::ffi::CString;

extern crate rconcmd;
use rconcmd::srcds::rcon::{Packet, PacketType};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:27015").expect("can't connect");
    let packet = Packet {
        id: 0,
        net_type: PacketType::SERVERDATA_AUTH,
        body: CString::new("12345").unwrap()
    };

    let _ = stream.write(&packet.serialize()).expect("can't send auth");
    let mut buff: Vec<u8> = vec![0; 4 + 4 + 4 + 2];
    let _ = stream.read(&mut buff).expect("can't read anything"); // ignore here too
    println!("{:?}", buff);

    let packet = Packet {
        id: 1,
        net_type: PacketType::SERVERDATA_EXECCOMMAND,
        body: CString::new("status").unwrap()
    };
    let _ = stream.write(&packet.serialize()).expect("can't send auth");

    let mut buff: Vec<u8> = vec![0; 128];
    let _ = stream.read(&mut buff).expect("can't read anything"); // ignore here too
    println!("{:?}", String::from_utf8(buff).unwrap());
}
