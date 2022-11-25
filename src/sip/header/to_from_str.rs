use core::fmt;

use simple_error::{bail, SimpleError};

use crate::strlit;

use super::Header;

impl Header {
    pub fn from_string(string: &str) -> Result<Header, SimpleError> {
        match string.to_lowercase().as_str() {
            "accept" => Ok(Header::Accept),
            "accept-encoding" => Ok(Header::AcceptEncoding),
            "accept-language" => Ok(Header::AcceptLanguage),
            "alert-info" => Ok(Header::AlertInfo),
            "allow" => Ok(Header::Allow),
            "authentication-info" => Ok(Header::AuthenticationInfo),
            "authorization" => Ok(Header::Authorization),
            "call-id" | "i" => Ok(Header::CallID),
            "call-info" => Ok(Header::CallInfo),
            "contact" | "m" => Ok(Header::Contact),
            "content-disposition" => Ok(Header::ContentDisposition),
            "content-encoding" | "e" => Ok(Header::ContentEncoding),
            "content-language" => Ok(Header::ContentLanguage),
            "content-length" | "l" => Ok(Header::ContentLength),
            "content-type" | "c" => Ok(Header::ContentType),
            "cseq" => Ok(Header::CSeq),
            "date" => Ok(Header::Date),
            "error-info" => Ok(Header::ErrorInfo),
            "expires" => Ok(Header::Expires),
            "from" | "f" => Ok(Header::From),
            "in-reply-to" => Ok(Header::InReplyTo),
            "max-forwards" => Ok(Header::MaxForwards),
            "mime-version" => Ok(Header::MIMEVersion),
            "min-expires" => Ok(Header::MinExpires),
            "organization" => Ok(Header::Organization),
            "priority" => Ok(Header::Priority),
            "proxy-authenticate" => Ok(Header::ProxyAuthenticate),
            "proxy-authorization" => Ok(Header::ProxyAuthorization),
            "proxy-require" => Ok(Header::ProxyRequire),
            "record-route" => Ok(Header::RecordRoute),
            "reply-to" => Ok(Header::ReplyTo),
            "require" => Ok(Header::Require),
            "retry-after" => Ok(Header::RetryAfter),
            "route" => Ok(Header::Route),
            "server" => Ok(Header::Server),
            "subject" | "s" => Ok(Header::Subject),
            "supported" => Ok(Header::Supported),
            "timestamp" => Ok(Header::Timestamp),
            "to" | "t" => Ok(Header::To),
            "unsupported" => Ok(Header::Unsupported),
            "user-agent" => Ok(Header::UserAgent),
            "via" | "k" => Ok(Header::Via),
            "warning" => Ok(Header::Warning),
            "www-authenticate" => Ok(Header::WWWAuthenticate),
            _ => bail!("Bad Header: {}", string),
        }
    }

    pub fn canonical_string(&self, compact: bool) -> String {
        if compact {
            match self {
                Header::CallID => strlit!("i"),
                Header::Contact => strlit!("m"),
                Header::ContentEncoding => strlit!("e"),
                Header::ContentLength => strlit!("l"),
                Header::ContentType => strlit!("c"),
                Header::From => strlit!("f"),
                Header::Subject => strlit!("s"),
                Header::Supported => strlit!("k"),
                Header::To => strlit!("t"),
                Header::Via => strlit!("v"),
                _ => self.to_string(),
            }
        } else {
            self.to_string()
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Header::Accept => write!(f, "Accept"),
            Header::AcceptEncoding => write!(f, "Accept-Encoding"),
            Header::AcceptLanguage => write!(f, "Accept-Language"),
            Header::AlertInfo => write!(f, "Alert-Info"),
            Header::Allow => write!(f, "Allow"),
            Header::AuthenticationInfo => write!(f, "AuthenticationInfo"),
            Header::Authorization => write!(f, "Authorization"),
            Header::CallID => write!(f, "Call-ID"),
            Header::CallInfo => write!(f, "Call-Info"),
            Header::Contact => write!(f, "Contact"),
            Header::ContentDisposition => write!(f, "Content-Disposition"),
            Header::ContentEncoding => write!(f, "Content-Encoding"),
            Header::ContentLanguage => write!(f, "Content-Language"),
            Header::ContentLength => write!(f, "Content-Length"),
            Header::ContentType => write!(f, "Content-Type"),
            Header::CSeq => write!(f, "CSeq"),
            Header::Date => write!(f, "Date"),
            Header::ErrorInfo => write!(f, "Error-Info"),
            Header::Expires => write!(f, "Expires"),
            Header::From => write!(f, "From"),
            Header::InReplyTo => write!(f, "In-Reply-To"),
            Header::MaxForwards => write!(f, "Max-Forwards"),
            Header::MIMEVersion => write!(f, "MIME-Version"),
            Header::MinExpires => write!(f, "Min-Expires"),
            Header::Organization => write!(f, "Organization"),
            Header::Priority => write!(f, "Priority"),
            Header::ProxyAuthenticate => write!(f, "Proxy-Authenticate"),
            Header::ProxyAuthorization => write!(f, "Proxy-Authorization"),
            Header::ProxyRequire => write!(f, "Proxy-Require"),
            Header::RecordRoute => write!(f, "Record-Route"),
            Header::ReplyTo => write!(f, "Reply-To"),
            Header::Require => write!(f, "Require"),
            Header::RetryAfter => write!(f, "Retry-After"),
            Header::Route => write!(f, "Route"),
            Header::Server => write!(f, "Server"),
            Header::Subject => write!(f, "Subject"),
            Header::Supported => write!(f, "Supported"),
            Header::Timestamp => write!(f, "Timestamp"),
            Header::To => write!(f, "To"),
            Header::Unsupported => write!(f, "Unsupported"),
            Header::UserAgent => write!(f, "User-Agent"),
            Header::Via => write!(f, "Via"),
            Header::Warning => write!(f, "Warning"),
            Header::WWWAuthenticate => write!(f, "WWW-Authenticate"),
        }
    }
}
