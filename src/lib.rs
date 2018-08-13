#![cfg_attr(feature = "strict", deny(warnings))]

extern crate bytes;
extern crate byteorder;
extern crate tokio;
extern crate tokio_io;
extern crate futures;
extern crate tokio_dns;
extern crate tokio_game_protocols;
#[macro_use] extern crate log;

pub mod srcds;
pub mod utils;
