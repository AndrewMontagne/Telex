use core::fmt;

use simple_error::{bail, SimpleError};

use super::SipHeader;

impl SipHeader {
    pub fn from_string(string: &str) -> Result<SipHeader, SimpleError> {
        match string.to_lowercase().as_str() {
            "accept" => Ok(SipHeader::Accept),
            "accept-encoding" => Ok(SipHeader::AcceptEncoding),
            "accept-language" => Ok(SipHeader::AcceptLanguage),
            "alert-info" => Ok(SipHeader::AlertInfo),
            "allow" => Ok(SipHeader::Allow),
            "authentication-info" => Ok(SipHeader::AuthenticationInfo),
            "authorization" => Ok(SipHeader::Authorization),
            "call-id" => Ok(SipHeader::CallID),
            "call-info" => Ok(SipHeader::CallInfo),
            "contact" => Ok(SipHeader::Contact),
            "content-disposition" => Ok(SipHeader::ContentDisposition),
            "content-encoding" => Ok(SipHeader::ContentEncoding),
            "content-language" => Ok(SipHeader::ContentLanguage),
            "content-length" => Ok(SipHeader::ContentLength),
            "content-type" => Ok(SipHeader::ContentType),
            "cseq" => Ok(SipHeader::CSeq),
            "date" => Ok(SipHeader::Date),
            "error-info" => Ok(SipHeader::ErrorInfo),
            "expires" => Ok(SipHeader::Expires),
            "from" => Ok(SipHeader::From),
            "in-reply-to" => Ok(SipHeader::InReplyTo),
            "max-forwards" => Ok(SipHeader::MaxForwards),
            "mime-version" => Ok(SipHeader::MIMEVersion),
            "min-expires" => Ok(SipHeader::MinExpires),
            "organization" => Ok(SipHeader::Organization),
            "priority" => Ok(SipHeader::Priority),
            "proxy-authenticate" => Ok(SipHeader::ProxyAuthenticate),
            "proxy-authorization" => Ok(SipHeader::ProxyAuthorization),
            "proxy-require" => Ok(SipHeader::ProxyRequire),
            "record-route" => Ok(SipHeader::RecordRoute),
            "reply-to" => Ok(SipHeader::ReplyTo),
            "require" => Ok(SipHeader::Require),
            "retry-after" => Ok(SipHeader::RetryAfter),
            "route" => Ok(SipHeader::Route),
            "server" => Ok(SipHeader::Server),
            "subject" => Ok(SipHeader::Subject),
            "supported" => Ok(SipHeader::Supported),
            "timestamp" => Ok(SipHeader::Timestamp),
            "to" => Ok(SipHeader::To),
            "unsupported" => Ok(SipHeader::Unsupported),
            "user-agent" => Ok(SipHeader::UserAgent),
            "via" => Ok(SipHeader::Via),
            "warning" => Ok(SipHeader::Warning),
            "www-authenticate" => Ok(SipHeader::WWWAuthenticate),
            _ => bail!("Bad Header: {}", string),
        }
    }

    pub fn to_string_with_value(&self, value: &String) {
        format!("{}: {}\r\n", self, value);
    }
}

impl fmt::Display for SipHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SipHeader::Accept => write!(f, "Accept"),
            SipHeader::AcceptEncoding => write!(f, "Accept-Encoding"),
            SipHeader::AcceptLanguage => write!(f, "Accept-Language"),
            SipHeader::AlertInfo => write!(f, "Alert-Info"),
            SipHeader::Allow => write!(f, "Allow"),
            SipHeader::AuthenticationInfo => write!(f, "AuthenticationInfo"),
            SipHeader::Authorization => write!(f, "Authorization"),
            SipHeader::CallID => write!(f, "Call-ID"),
            SipHeader::CallInfo => write!(f, "Call-Info"),
            SipHeader::Contact => write!(f, "Contact"),
            SipHeader::ContentDisposition => write!(f, "Content-Disposition"),
            SipHeader::ContentEncoding => write!(f, "Content-Encoding"),
            SipHeader::ContentLanguage => write!(f, "Content-Language"),
            SipHeader::ContentLength => write!(f, "Content-Length"),
            SipHeader::ContentType => write!(f, "Content-Type"),
            SipHeader::CSeq => write!(f, "CSeq"),
            SipHeader::Date => write!(f, "Date"),
            SipHeader::ErrorInfo => write!(f, "Error-Info"),
            SipHeader::Expires => write!(f, "Expires"),
            SipHeader::From => write!(f, "From"),
            SipHeader::InReplyTo => write!(f, "In-Reply-To"),
            SipHeader::MaxForwards => write!(f, "Max-Forwards"),
            SipHeader::MIMEVersion => write!(f, "MIME-Version"),
            SipHeader::MinExpires => write!(f, "Min-Expires"),
            SipHeader::Organization => write!(f, "Organization"),
            SipHeader::Priority => write!(f, "Priority"),
            SipHeader::ProxyAuthenticate => write!(f, "Proxy-Authenticate"),
            SipHeader::ProxyAuthorization => write!(f, "Proxy-Authorization"),
            SipHeader::ProxyRequire => write!(f, "Proxy-Require"),
            SipHeader::RecordRoute => write!(f, "Record-Route"),
            SipHeader::ReplyTo => write!(f, "Reply-To"),
            SipHeader::Require => write!(f, "Require"),
            SipHeader::RetryAfter => write!(f, "RetryAfter"),
            SipHeader::Route => write!(f, "Route"),
            SipHeader::Server => write!(f, "Server"),
            SipHeader::Subject => write!(f, "Subject"),
            SipHeader::Supported => write!(f, "Supported"),
            SipHeader::Timestamp => write!(f, "Timestamp"),
            SipHeader::To => write!(f, "To"),
            SipHeader::Unsupported => write!(f, "Unsupported"),
            SipHeader::UserAgent => write!(f, "User-Agent"),
            SipHeader::Via => write!(f, "Via"),
            SipHeader::Warning => write!(f, "Warning"),
            SipHeader::WWWAuthenticate => write!(f, "WWW-Authenticate"),
        }
    }
}
