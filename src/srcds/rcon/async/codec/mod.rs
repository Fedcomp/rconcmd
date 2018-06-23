mod encoder;
mod decoder;

#[derive(Debug)]
pub struct Codec {
    packet_id_increment: i32
}

impl Codec {
    pub fn new() -> Codec {
        Codec { packet_id_increment: 0 }
    }
}
