pub enum PacketDirection {
    INCOMING,
    OUTCOMING,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum PacketType {
    SERVERDATA_AUTH,
    SERVERDATA_AUTH_RESPONSE,
    SERVERDATA_EXECCOMMAND,
    SERVERDATA_RESPONSE_VALUE,
}

impl PacketType {
    pub fn from_value(raw_type: i32, direction: PacketDirection) -> PacketType {
        use self::PacketType::*;
        use self::PacketDirection::*;

        match raw_type {
            3 => SERVERDATA_AUTH,
            2 => {
                match direction {
                    INCOMING => SERVERDATA_AUTH_RESPONSE,
                    OUTCOMING => SERVERDATA_EXECCOMMAND,
                }
            }
            _ => SERVERDATA_RESPONSE_VALUE,
        }
    }

    pub fn value(&self) -> i32 {
        use self::PacketType::*;

        match *self {
            SERVERDATA_AUTH => 3,
            SERVERDATA_AUTH_RESPONSE => 2,
            SERVERDATA_EXECCOMMAND => 2,
            SERVERDATA_RESPONSE_VALUE => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::PacketType::*;
    use super::PacketDirection::*;

    #[test]
    fn test_from_value() {
        assert_eq!(PacketType::from_value(3, OUTCOMING), SERVERDATA_AUTH);
        assert_eq!(
            PacketType::from_value(2, INCOMING),
            SERVERDATA_AUTH_RESPONSE
        );
        assert_eq!(PacketType::from_value(2, OUTCOMING), SERVERDATA_EXECCOMMAND);
        assert_eq!(
            PacketType::from_value(0, OUTCOMING),
            SERVERDATA_RESPONSE_VALUE
        );
    }

    #[test]
    fn test_value() {
        assert_eq!(PacketType::SERVERDATA_AUTH.value(), 3);
        assert_eq!(PacketType::SERVERDATA_AUTH_RESPONSE.value(), 2);
        assert_eq!(PacketType::SERVERDATA_EXECCOMMAND.value(), 2);
        assert_eq!(PacketType::SERVERDATA_RESPONSE_VALUE.value(), 0);
    }
}
