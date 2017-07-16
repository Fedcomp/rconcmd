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
    let response_packet = Packet::read_from(&mut stream).unwrap();

    // if id == 255 == failed
    let command_packet = Packet {
        id: 0,
        net_type: PacketType::SERVERDATA_EXECCOMMAND,
        body: CString::new("echo status").unwrap()
    };
    let _ = stream.write(&command_packet.serialize()).expect("can't send command");

    let response_packet = Packet::read_from(&mut stream).unwrap();
    println!("{:?}", response_packet);
}
