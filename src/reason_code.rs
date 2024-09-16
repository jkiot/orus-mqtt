use std::convert::TryFrom;

const ERROR_CODE: &'static str = "Unsupported Reason Code";

#[derive(Debug, PartialEq)]
pub enum ReasonCode {
    Success = 0x00, // Also used for : Normal disconnection , Granted QoS 0
    GrantedQos1 = 0x01,
    GrantedQos2 = 0x02,
    DisconnectWithWillMessage = 0x04,
    NoMatchingSubscribers = 0x10,
    NoSubscriptionExisted = 0x11,
    ContinueAuthentication = 0x18,
    ReAuthentication = 0x19,
    UnspecifiedError = 0x80,
    MalformedPacket = 0x81,
    ProtocolError = 0x82,
    ImplementationSpecificError = 0x83,
    UnsupportedProtocolVersion = 0x84,
    ClientIdentifierNotValid = 0x85,
    BadUserNameOrPassword = 0x86,
    NotAuthorized = 0x87,
    ServerUnavailable = 0x88,
    ServerBusy = 0x89,
    Banned = 0x8A,
    ServerShuttingDown = 0x8B,
    BadAuthenticationMethod = 0x8C,
    KeepAliveTimeout = 0x8D,
    SessionTakeOver = 0x8E,
    TopicFilterInvalid = 0x8F,
    TopicNameInvalid = 0x90,
    PacketIdentifierInUse = 0x91,
    PacketIdentifierNotFound = 0x92,
    ReceiveMaximumExceeded = 0x93,
    TopicAliasInvalid = 0x94,
    PacketTooLarge = 0x95,
    MassageRateTooHigh = 0x96,
    QuotaExceeded = 0x97,
    AdministrativeAction = 0x98,
    PayloadFormatInvalid = 0x99,
    RetainNotSupported = 0x9A,
    QosNotSupported = 0x9B,
    UseAnotherServer = 0x9C,
    ServerMoved = 0x9D,
    SharedSubscriptionsNotSupported = 0x9E,
    ConnectionRateExceeded = 0x9F,
    MaximumConnectedTime = 0xA0,
    SubscriptionIdentifiersNotSupported = 0xA1,
    WildcardSubscriptionNotSupported = 0xA2,
}

impl TryFrom<u8> for ReasonCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Success),
            0x01 => Ok(Self::GrantedQos1),
            0x02 => Ok(Self::GrantedQos2),
            0x04 => Ok(Self::DisconnectWithWillMessage),
            0x10 => Ok(Self::NoMatchingSubscribers),
            0x11 => Ok(Self::NoSubscriptionExisted),
            0x18 => Ok(Self::ContinueAuthentication),
            0x19 => Ok(Self::ReAuthentication),
            0x80 => Ok(Self::UnspecifiedError),
            0x81 => Ok(Self::MalformedPacket),
            0x82 => Ok(Self::ProtocolError),
            0x83 => Ok(Self::ImplementationSpecificError),
            0x84 => Ok(Self::UnsupportedProtocolVersion),
            0x85 => Ok(Self::ClientIdentifierNotValid),
            0x86 => Ok(Self::BadUserNameOrPassword),
            0x87 => Ok(Self::NotAuthorized),
            0x88 => Ok(Self::ServerUnavailable),
            0x89 => Ok(Self::ServerBusy),
            0x8A => Ok(Self::Banned),
            0x8B => Ok(Self::ServerShuttingDown),
            0x8C => Ok(Self::BadAuthenticationMethod),
            0x8D => Ok(Self::KeepAliveTimeout),
            0x8E => Ok(Self::SessionTakeOver),
            0x8F => Ok(Self::TopicFilterInvalid),
            0x90 => Ok(Self::TopicNameInvalid),
            0x91 => Ok(Self::PacketIdentifierInUse),
            0x92 => Ok(Self::PacketIdentifierNotFound),
            0x93 => Ok(Self::ReceiveMaximumExceeded),
            0x94 => Ok(Self::TopicAliasInvalid),
            0x95 => Ok(Self::PacketTooLarge),
            0x96 => Ok(Self::MassageRateTooHigh),
            0x97 => Ok(Self::QuotaExceeded),
            0x98 => Ok(Self::AdministrativeAction),
            0x99 => Ok(Self::PayloadFormatInvalid),
            0x9A => Ok(Self::RetainNotSupported),
            0x9B => Ok(Self::QosNotSupported),
            0x9C => Ok(Self::UseAnotherServer),
            0x9D => Ok(Self::ServerMoved),
            0x9E => Ok(Self::SharedSubscriptionsNotSupported),
            0x9F => Ok(Self::ConnectionRateExceeded),
            0xA0 => Ok(Self::MaximumConnectedTime),
            0xA1 => Ok(Self::SubscriptionIdentifiersNotSupported),
            0xA2 => Ok(Self::WildcardSubscriptionNotSupported),
            _ => Err(ERROR_CODE),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_correct_mapping_from_value_to_reason_code() {
        let mut test_data = Vec::new();

        test_data.push((0, Result::Ok(ReasonCode::Success)));
        test_data.push((1, Result::Ok(ReasonCode::GrantedQos1)));
        test_data.push((2, Result::Ok(ReasonCode::GrantedQos2)));
        test_data.push((4, Result::Ok(ReasonCode::DisconnectWithWillMessage)));
        test_data.push((16, Result::Ok(ReasonCode::NoMatchingSubscribers)));
        test_data.push((17, Result::Ok(ReasonCode::NoSubscriptionExisted)));
        test_data.push((24, Result::Ok(ReasonCode::ContinueAuthentication)));
        test_data.push((25, Result::Ok(ReasonCode::ReAuthentication)));
        test_data.push((128, Result::Ok(ReasonCode::UnspecifiedError)));
        test_data.push((129, Result::Ok(ReasonCode::MalformedPacket)));
        test_data.push((130, Result::Ok(ReasonCode::ProtocolError)));
        test_data.push((131, Result::Ok(ReasonCode::ImplementationSpecificError)));
        test_data.push((132, Result::Ok(ReasonCode::UnsupportedProtocolVersion)));
        test_data.push((133, Result::Ok(ReasonCode::ClientIdentifierNotValid)));
        test_data.push((134, Result::Ok(ReasonCode::BadUserNameOrPassword)));
        test_data.push((135, Result::Ok(ReasonCode::NotAuthorized)));
        test_data.push((136, Result::Ok(ReasonCode::ServerUnavailable)));
        test_data.push((137, Result::Ok(ReasonCode::ServerBusy)));
        test_data.push((138, Result::Ok(ReasonCode::Banned)));
        test_data.push((139, Result::Ok(ReasonCode::ServerShuttingDown)));
        test_data.push((140, Result::Ok(ReasonCode::BadAuthenticationMethod)));
        test_data.push((141, Result::Ok(ReasonCode::KeepAliveTimeout)));
        test_data.push((142, Result::Ok(ReasonCode::SessionTakeOver)));
        test_data.push((143, Result::Ok(ReasonCode::TopicFilterInvalid)));
        test_data.push((144, Result::Ok(ReasonCode::TopicNameInvalid)));
        test_data.push((145, Result::Ok(ReasonCode::PacketIdentifierInUse)));
        test_data.push((146, Result::Ok(ReasonCode::PacketIdentifierNotFound)));
        test_data.push((147, Result::Ok(ReasonCode::ReceiveMaximumExceeded)));
        test_data.push((148, Result::Ok(ReasonCode::TopicAliasInvalid)));
        test_data.push((149, Result::Ok(ReasonCode::PacketTooLarge)));
        test_data.push((150, Result::Ok(ReasonCode::MassageRateTooHigh)));
        test_data.push((151, Result::Ok(ReasonCode::QuotaExceeded)));
        test_data.push((152, Result::Ok(ReasonCode::AdministrativeAction)));
        test_data.push((153, Result::Ok(ReasonCode::PayloadFormatInvalid)));
        test_data.push((154, Result::Ok(ReasonCode::RetainNotSupported)));
        test_data.push((155, Result::Ok(ReasonCode::QosNotSupported)));
        test_data.push((156, Result::Ok(ReasonCode::UseAnotherServer)));
        test_data.push((157, Result::Ok(ReasonCode::ServerMoved)));
        test_data.push((158, Result::Ok(ReasonCode::SharedSubscriptionsNotSupported)));
        test_data.push((159, Result::Ok(ReasonCode::ConnectionRateExceeded)));
        test_data.push((160, Result::Ok(ReasonCode::MaximumConnectedTime)));
        test_data.push((
            161,
            Result::Ok(ReasonCode::SubscriptionIdentifiersNotSupported),
        ));
        test_data.push((
            162,
            Result::Ok(ReasonCode::WildcardSubscriptionNotSupported),
        ));
        test_data.push((15, Result::Err(ERROR_CODE)));
        test_data.push((163, Result::Err(ERROR_CODE)));

        test_data
            .into_iter()
            .enumerate()
            .for_each(|(test_case, (value, expected_result))| {
                println!("Running Test Case: {}", test_case);

                let reason_code = ReasonCode::try_from(value);
                assert_eq!(expected_result, reason_code);
            });
    }
}
