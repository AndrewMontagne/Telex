use chrono::Utc;

use super::SipHeader;

impl SipHeader {
    pub fn generate(self) -> String {
        match self {
            Self::Date => Utc::now().to_rfc2822(),
            Self::Server => format!("Telex PBX v{}", env!("CARGO_PKG_VERSION")),
            _ => "".to_string(),
        }
    }
}
