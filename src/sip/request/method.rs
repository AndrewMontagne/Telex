use simple_error::{bail, SimpleError};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Method {
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

impl Method {
    pub fn from_string(string: &String) -> Result<Method, SimpleError> {
        match string.to_uppercase().as_str() {
            "INVITE" => Ok(Method::Invite),
            "ACK" => Ok(Method::Ack),
            "OPTIONS" => Ok(Method::Options),
            "BYE" => Ok(Method::Bye),
            "CANCEL" => Ok(Method::Cancel),
            "REGISTER" => Ok(Method::Register),
            "NOTIFY" => Ok(Method::Notify),
            "PUBLISH" => Ok(Method::Publish),
            "SUBSCRIBE" => Ok(Method::Subscribe),
            _ => bail!("Unknown Request Method: {}", string),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::Invite => write!(f, "INVITE"),
            Method::Ack => write!(f, "ACK"),
            Method::Options => write!(f, "OPTIONS"),
            Method::Bye => write!(f, "BYE"),
            Method::Cancel => write!(f, "CANCEL"),
            Method::Register => write!(f, "REGISTER"),
            Method::Notify => write!(f, "NOTIFY"),
            Method::Publish => write!(f, "PUBLISH"),
            Method::Subscribe => write!(f, "SUBSCRIBE"),
        }
    }
}
