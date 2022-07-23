use std::convert::TryFrom;

use crate::reason_code::ReasonCode;

#[derive(Debug, PartialEq)]
pub enum PacketType {
    CONNECT = 1,
    CONNACK,
    PUBLISH,
    PUBACK,
    PUBREC,
    PUBREL,
    PUBCOMP,
    SUBSCRIBE,
    SUBACK,
    UNSUBSCRIBE,
    UNSUBACK,
    PINGREQ,
    PINGRESP,
    DISCONNECT,
    AUTH,
}

impl TryFrom<u8> for PacketType {
    type Error = ReasonCode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PacketType::CONNECT),
            2 => Ok(PacketType::CONNACK),
            3 => Ok(PacketType::PUBLISH),
            4 => Ok(PacketType::PUBACK),
            5 => Ok(PacketType::PUBREC),
            6 => Ok(PacketType::PUBREL),
            7 => Ok(PacketType::PUBCOMP),
            8 => Ok(PacketType::SUBSCRIBE),
            9 => Ok(PacketType::SUBACK),
            10 => Ok(PacketType::UNSUBSCRIBE),
            11 => Ok(PacketType::UNSUBACK),
            12 => Ok(PacketType::PINGREQ),
            13 => Ok(PacketType::PINGRESP),
            14 => Ok(PacketType::DISCONNECT),
            15 => Ok(PacketType::AUTH),
            _ => Err(ReasonCode::MalformedPacket),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_that_correct_mapping_from_value_to_packet_type_is_done() {
        let mut test_data = Vec::new();
        test_data.push((0, Result::Err(ReasonCode::MalformedPacket)));
        test_data.push((1, Result::Ok(PacketType::CONNECT)));
        test_data.push((2, Result::Ok(PacketType::CONNACK)));
        test_data.push((3, Result::Ok(PacketType::PUBLISH)));
        test_data.push((4, Result::Ok(PacketType::PUBACK)));
        test_data.push((5, Result::Ok(PacketType::PUBREC)));
        test_data.push((6, Result::Ok(PacketType::PUBREL)));
        test_data.push((7, Result::Ok(PacketType::PUBCOMP)));
        test_data.push((8, Result::Ok(PacketType::SUBSCRIBE)));
        test_data.push((9, Result::Ok(PacketType::SUBACK)));
        test_data.push((10, Result::Ok(PacketType::UNSUBSCRIBE)));
        test_data.push((11, Result::Ok(PacketType::UNSUBACK)));
        test_data.push((12, Result::Ok(PacketType::PINGREQ)));
        test_data.push((13, Result::Ok(PacketType::PINGRESP)));
        test_data.push((14, Result::Ok(PacketType::DISCONNECT)));
        test_data.push((15, Result::Ok(PacketType::AUTH)));
        test_data.push((15, Result::Ok(PacketType::AUTH)));
        test_data.push((16, Result::Err(ReasonCode::MalformedPacket)));

        test_data
            .into_iter()
            .enumerate()
            .for_each(|(test_case, (value, expected_result))| {
                println!("Running Test Case: {}", test_case);
                assert_eq!(expected_result, PacketType::try_from(value));
            });
    }
}
