use std::fmt;

use super::Status;

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::None => write!(f, "No Response"),

            // Informational
            Status::Trying => write!(f, "100 Trying"),
            Status::Ringing => write!(f, "180 Ringing"),
            Status::CallIsBeingForwarded => write!(f, "181 Call Is Being Forwarded"),
            Status::Queued => write!(f, "182 Queued"),
            Status::SessionProgress => write!(f, "183 Session Progress"),

            // OK
            Status::OK => write!(f, "200 OK"),

            // Redirection
            Status::MultipleChoices => write!(f, "300 Multiple Choices"),
            Status::MovedPermanently => write!(f, "301 Moved Permanently"),
            Status::MovedTemporarily => write!(f, "302 Moved Temporarily"),
            Status::UseProxy => write!(f, "305 Use Proxy"),
            Status::AlternativeService => write!(f, "380 Alternative Service"),

            // Client Error
            Status::BadRequest => write!(f, "400 Bad Request"),
            Status::Unauthorized => write!(f, "401 Unauthorized"),
            Status::PaymentRequired => write!(f, "402 Payment Required"),
            Status::Forbidden => write!(f, "403 Forbidden"),
            Status::NotFound => write!(f, "404 NotFound"),
            Status::MethodNotAllowed => write!(f, "405 Method Not Allowed"),
            Status::NotAcceptable => write!(f, "406 Not Acceptable"),
            Status::ProxyAuthenticationRequired => write!(f, "407 Proxy Authentication Required"),
            Status::RequestTimeout => write!(f, "408 Request Timeout"),
            Status::Gone => write!(f, "410 Gone"),
            Status::RequestEntityTooLarge => write!(f, "413 Request Entity Too Large"),
            Status::RequestUriTooLarge => write!(f, "414 Request Uri Too Large"),
            Status::UnsupportedMediaType => write!(f, "415 Unsupported Media Type"),
            Status::UnsupportedURIScheme => write!(f, "416 Unsupported URI Scheme"),
            Status::BadExtension => write!(f, "420 Bad Extension"),
            Status::ExtensionRequired => write!(f, "421 Extension Required"),
            Status::IntervalTooBrief => write!(f, "423 Interval Too Brief"),
            Status::TemporarilyNotAvailable => write!(f, "480 Temporarily Not Available"),
            Status::CallTransactionDoesNotExist => write!(f, "481 Call/Transaction Does Not Exist"),
            Status::LoopDetected => write!(f, "482 Loop Detected"),
            Status::TooManyHops => write!(f, "483 Too Many Hops"),
            Status::AddressIncomplete => write!(f, "484 Address Incomplete"),
            Status::Ambiguous => write!(f, "485 Ambiguous"),
            Status::BusyHere => write!(f, "486 Busy"),
            Status::RequestTerminated => write!(f, "487 Request Terminated"),
            Status::NotAcceptableHere => write!(f, "488 Not Acceptable Here"),
            Status::RequestPending => write!(f, "491 Request Pending"),
            Status::Undecipherable => write!(f, "493 Undecipherable"),

            // Server Error
            Status::InternalServerError => write!(f, "500 Internal Server Error"),
            Status::NotImplemented => write!(f, "501 Not Implemented"),
            Status::BadGateway => write!(f, "502 Bad Gateway"),
            Status::ServiceUnavailable => write!(f, "503 Service Unavailable"),
            Status::ServerTimeOut => write!(f, "504 Server Time Out"),
            Status::SipVersionNotSupported => write!(f, "505 SIP Version Not Supported"),
            Status::MessageTooLarge => write!(f, "513 Message Too Large"),

            // Global Failure
            Status::BusyEverywhere => write!(f, "600 Busy Everywhere"),
            Status::Decline => write!(f, "603 Declined"),
            Status::DoesNotExistAnywhere => write!(f, "604 Does Not Exist Anywhere"),
            Status::GlobalNotAcceptable => write!(f, "606 Not Acceptable"),
        }
    }
}