#![allow(dead_code)]

pub mod to_from_str;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Status {
    // No response, send no packets
    None = 0,

    // Informational
    Trying = 100,
    Ringing = 180,
    CallIsBeingForwarded = 181,
    Queued = 182,
    SessionProgress = 183,

    // Success
    OK = 200,

    // Redirection
    MultipleChoices = 300,
    MovedPermanently = 301,
    MovedTemporarily = 302,
    UseProxy = 305,
    AlternativeService = 380,

    // Client Error
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Gone = 410,
    RequestEntityTooLarge = 413,
    RequestUriTooLarge = 414,
    UnsupportedMediaType = 415,
    UnsupportedURIScheme = 416,
    BadExtension = 420,
    ExtensionRequired = 421,
    IntervalTooBrief = 423,
    TemporarilyNotAvailable = 480,
    CallTransactionDoesNotExist = 481,
    LoopDetected = 482,
    TooManyHops = 483,
    AddressIncomplete = 484,
    Ambiguous = 485,
    BusyHere = 486,
    RequestTerminated = 487,
    NotAcceptableHere = 488,
    RequestPending = 491,
    Undecipherable = 493,

    // Server Error
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    ServerTimeOut = 504,
    SipVersionNotSupported = 505,
    MessageTooLarge = 513,

    // Global Failure
    BusyEverywhere = 600,
    Decline = 603,
    DoesNotExistAnywhere = 604,
    GlobalNotAcceptable = 606,
}
