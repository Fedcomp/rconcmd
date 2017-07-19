use std::io::Error;
use std::io;
use std::io::BufRead;

extern crate rconcmd;
use rconcmd::srcds::rcon::Connection;

fn main() {
    let mut connection = Connection::new("127.0.0.1:27015", "12345").unwrap();

    loop {
        let command = read_from_stdin().unwrap();
        let res = connection.send_command(&command).unwrap();
        println!("{}", res.to_str().unwrap());
    }
}

fn read_from_stdin() -> Result<String, Error> {
    let mut input = String::new();
    let stdin = io::stdin();
    print!("> ");
    stdin.lock().read_line(&mut input)?;
    Ok(input)
}
