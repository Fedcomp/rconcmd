#![cfg_attr(feature = "strict", deny(warnings))]

extern crate clap;
extern crate rconcmd;
extern crate tokio;
extern crate tokio_dns;
extern crate futures;

use clap::{Arg, App};
use futures::Future;

use rconcmd::srcds::rcon::AsyncConnection;

fn main() {
    let matches = App::new("async_rconcmd")
        .version("0.2.0")
        .author("Fedcomp")
        .about("Asynchronous rcon console for srcds servers")
        .arg(Arg::with_name("hostname").required(true).takes_value(true))
        .arg(
            Arg::with_name("rcon")
                .help("rcon_password of the server")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let hostname = matches.value_of("hostname").unwrap();
    let rcon_password = matches.value_of("rcon").unwrap();
    let connection = AsyncConnection::connect(hostname, rcon_password).map_err(|err| {
        println!("err = {:?}", err);
    });

    tokio::run(connection);
}
