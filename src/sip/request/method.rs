use simple_error::{SimpleError, bail};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SipRequestMethod {
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

impl SipRequestMethod {
    pub fn from_string(string: String) -> Result<SipRequestMethod, SimpleError> {
        match string.to_uppercase().as_str() {
            "INVITE" => Ok(SipRequestMethod::Invite),
            "ACK" => Ok(SipRequestMethod::Ack),
            "OPTIONS" => Ok(SipRequestMethod::Options),
            "BYE" => Ok(SipRequestMethod::Bye),
            "CANCEL" => Ok(SipRequestMethod::Cancel),
            "REGISTER" => Ok(SipRequestMethod::Register),
            "NOTIFY" => Ok(SipRequestMethod::Notify),
            "PUBLISH" => Ok(SipRequestMethod::Publish),
            "SUBSCRIBE" => Ok(SipRequestMethod::Subscribe),
            _ => bail!("Unknown Request Method: {}", string),
        }
    }
}

impl fmt::Display for SipRequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SipRequestMethod::Invite => write!(f, "INVITE"),
            SipRequestMethod::Ack => write!(f, "ACK"),
            SipRequestMethod::Options => write!(f, "OPTIONS"),
            SipRequestMethod::Bye => write!(f, "BYE"),
            SipRequestMethod::Cancel => write!(f, "INVITE"),
            SipRequestMethod::Register => write!(f, "REGISTER"),
            SipRequestMethod::Notify => write!(f, "NOTIFY"),
            SipRequestMethod::Publish => write!(f, "PUBLISH"),
            SipRequestMethod::Subscribe => write!(f, "SUBSCRIBE"),
        }
    }
}
