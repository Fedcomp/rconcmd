pub const OUTCOMING_PACKET: bool = true;
pub const INCOMING_PACKET:  bool = false;

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum PacketType {
    SERVERDATA_AUTH,
    SERVERDATA_AUTH_RESPONSE,
    SERVERDATA_EXECCOMMAND,
    SERVERDATA_RESPONSE_VALUE
}

impl PacketType {
    pub fn from_value(raw_type: i32, outcoming: bool) -> PacketType {
        use self::PacketType::*;

        match raw_type {
            3 => SERVERDATA_AUTH,
            2 => { if outcoming { SERVERDATA_EXECCOMMAND } else { SERVERDATA_AUTH_RESPONSE } },
            _ => SERVERDATA_RESPONSE_VALUE
        }
    }

    pub fn value(&self) -> i32 {
        use self::PacketType::*;

        match *self {
            SERVERDATA_AUTH => 3,
            SERVERDATA_AUTH_RESPONSE => 2,
            SERVERDATA_EXECCOMMAND => 2,
            SERVERDATA_RESPONSE_VALUE => 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::PacketType::*;

    #[test]
    fn test_from_value() {
        assert_eq!(PacketType::from_value(3, OUTCOMING_PACKET), SERVERDATA_AUTH);
        assert_eq!(PacketType::from_value(2, INCOMING_PACKET ), SERVERDATA_AUTH_RESPONSE);
        assert_eq!(PacketType::from_value(2, OUTCOMING_PACKET), SERVERDATA_EXECCOMMAND);
        assert_eq!(PacketType::from_value(0, OUTCOMING_PACKET), SERVERDATA_RESPONSE_VALUE);
    }

    #[test]
    fn test_value() {
        assert_eq!(PacketType::SERVERDATA_AUTH.value(),           3);
        assert_eq!(PacketType::SERVERDATA_AUTH_RESPONSE.value(),  2);
        assert_eq!(PacketType::SERVERDATA_EXECCOMMAND.value(),    2);
        assert_eq!(PacketType::SERVERDATA_RESPONSE_VALUE.value(), 0);
    }
}
