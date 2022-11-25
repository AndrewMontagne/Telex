use simple_error::{bail, SimpleError};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SipMethod {
    Invite,
    Ack,
    Options,
    Bye,
    Cancel,
    Register,
    Notify,
    Publish,
    Subscribe,
}

impl SipMethod {
    pub fn from_string(string: &String) -> Result<SipMethod, SimpleError> {
        match string.to_uppercase().as_str() {
            "INVITE" => Ok(SipMethod::Invite),
            "ACK" => Ok(SipMethod::Ack),
            "OPTIONS" => Ok(SipMethod::Options),
            "BYE" => Ok(SipMethod::Bye),
            "CANCEL" => Ok(SipMethod::Cancel),
            "REGISTER" => Ok(SipMethod::Register),
            "NOTIFY" => Ok(SipMethod::Notify),
            "PUBLISH" => Ok(SipMethod::Publish),
            "SUBSCRIBE" => Ok(SipMethod::Subscribe),
            _ => bail!("Unknown Request Method: {}", string),
        }
    }
}

impl fmt::Display for SipMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SipMethod::Invite => write!(f, "INVITE"),
            SipMethod::Ack => write!(f, "ACK"),
            SipMethod::Options => write!(f, "OPTIONS"),
            SipMethod::Bye => write!(f, "BYE"),
            SipMethod::Cancel => write!(f, "CANCEL"),
            SipMethod::Register => write!(f, "REGISTER"),
            SipMethod::Notify => write!(f, "NOTIFY"),
            SipMethod::Publish => write!(f, "PUBLISH"),
            SipMethod::Subscribe => write!(f, "SUBSCRIBE"),
        }
    }
}
