use simple_error::{bail, SimpleError};

use super::SipHeader;

impl SipHeader {
    pub fn validate(&self, value: &String) -> Result<(), SimpleError> {
        match self {
            Self::Expires | Self::ContentLength | Self::MaxForwards => validate_integer(value),
            _ => Ok(()),
        }
    }
}

fn validate_integer(value: &String) -> Result<(), SimpleError> {
    match value.parse::<i32>() {
        Ok(_) => Ok(()),
        Err(_) => bail!("Could not parse '{}' to an int", value),
    }
}
