#[allow(non_camel_case_types)]
#[derive(Clone)]
#[derive(Copy)]
pub enum PacketType {
    SERVERDATA_AUTH = 3,
    // SERVERDATA_AUTH_RESPONSE,
    SERVERDATA_EXECCOMMAND = 2,
    SERVERDATA_RESPONSE_VALUE = 0
}
