use chrono::Utc;

use crate::strlit;

use super::Header;

impl Header {
    pub fn generate(self) -> String {
        match self {
            Self::Date => Utc::now().to_rfc2822(),
            Self::Server => format!("Telex PBX v{}", env!("CARGO_PKG_VERSION")),
            Self::Allow => strlit!("REGISTER, INVITE, ACK, CANCEL, OPTIONS, BYE"),
            _ => String::new(),
        }
    }
}
