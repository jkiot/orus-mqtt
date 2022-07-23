use std::convert::TryFrom;

use crate::reason_code::ReasonCode;

#[derive(Debug, PartialEq)]
pub enum PacketType {
    Connect = 1,
    Connack,
    Publish,
    PubAck,
    PubRec,
    PubRel,
    PubComp,
    Subscribe,
    SubAck,
    Unsubscribe,
    UnsubAck,
    PingReq,
    PingResp,
    Disconnect,
    Auth,
}

impl TryFrom<u8> for PacketType {
    type Error = ReasonCode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PacketType::Connect),
            2 => Ok(PacketType::Connack),
            3 => Ok(PacketType::Publish),
            4 => Ok(PacketType::PubAck),
            5 => Ok(PacketType::PubRec),
            6 => Ok(PacketType::PubRel),
            7 => Ok(PacketType::PubComp),
            8 => Ok(PacketType::Subscribe),
            9 => Ok(PacketType::SubAck),
            10 => Ok(PacketType::Unsubscribe),
            11 => Ok(PacketType::UnsubAck),
            12 => Ok(PacketType::PingReq),
            13 => Ok(PacketType::PingResp),
            14 => Ok(PacketType::Disconnect),
            15 => Ok(PacketType::Auth),
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
        test_data.push((1, Result::Ok(PacketType::Connect)));
        test_data.push((2, Result::Ok(PacketType::Connack)));
        test_data.push((3, Result::Ok(PacketType::Publish)));
        test_data.push((4, Result::Ok(PacketType::PubAck)));
        test_data.push((5, Result::Ok(PacketType::PubRec)));
        test_data.push((6, Result::Ok(PacketType::PubRel)));
        test_data.push((7, Result::Ok(PacketType::PubComp)));
        test_data.push((8, Result::Ok(PacketType::Subscribe)));
        test_data.push((9, Result::Ok(PacketType::SubAck)));
        test_data.push((10, Result::Ok(PacketType::Unsubscribe)));
        test_data.push((11, Result::Ok(PacketType::UnsubAck)));
        test_data.push((12, Result::Ok(PacketType::PingReq)));
        test_data.push((13, Result::Ok(PacketType::PingResp)));
        test_data.push((14, Result::Ok(PacketType::Disconnect)));
        test_data.push((15, Result::Ok(PacketType::Auth)));
        test_data.push((15, Result::Ok(PacketType::Auth)));
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
