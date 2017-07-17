extern crate rconcmd;
use rconcmd::srcds::rcon::Connection;

fn main() {
    let _connection = Connection::new("127.0.0.1:27015", "12345").unwrap();
}
