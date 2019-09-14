#![feature(async_await)]

use std::io::Error;
use std::io;
use std::io::{Write, BufRead};
use std::process;

use clap::{Arg, App};

use rconcmd::srcds::rcon::Connection;

#[tokio::main]
async fn main() {
    let matches = App::new("rconcmd")
                          .version("1.0")
                          .author("Fedcomp")
                          .about("Rcon console for srcds servers")
                          .arg(Arg::with_name("hostname")
                               .required(true)
                               .takes_value(true))
                          .arg(Arg::with_name("rcon")
                               .required(true)
                               .help("rcon_password of the server")
                               .takes_value(true))
                          .arg(Arg::with_name("execute")
                               .short("e")
                               .long("execute")
                               .help("Execute single command, print response and quit")
                               .takes_value(true))
                          .get_matches();

    let hostname = matches.value_of("hostname").unwrap();
    let rcon_password = matches.value_of("rcon").unwrap();

    let mut connection = Connection::new(hostname, rcon_password).unwrap();

    // Execute single command and exit if one is specified
    match matches.value_of("execute") {
        None => (),
        Some(cmd) => {
            let res = connection.send_command(cmd).unwrap();
            println!("{}", res.to_str().unwrap());
            process::exit(0);
        }
    }

    loop {
        let command = read_from_stdin().unwrap();
        let res = connection.send_command(&command).unwrap();
        println!("{}", res.to_str().unwrap());
    }
}

fn read_from_stdin() -> Result<String, Error> {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut input)?;
    Ok(input)
}
