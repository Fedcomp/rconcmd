#![cfg_attr(feature = "strict", deny(warnings))]

extern crate byteorder;
extern crate bytes;
extern crate futures;
extern crate tokio;
extern crate tokio_dns;
extern crate tokio_game_protocols;
extern crate tokio_io;
#[macro_use]
extern crate log;

pub mod srcds;
pub mod utils;
