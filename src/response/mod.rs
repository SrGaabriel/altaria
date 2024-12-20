pub mod into;

use std::fmt::Display;
use crate::request::HttpHeaderMap;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: HttpStatusCode,
    pub headers: HttpHeaderMap,
    pub body: Vec<u8>
}

impl HttpResponse {
    pub fn empty(status: HttpStatusCode) -> HttpResponse {
        HttpResponse {
            status_code: HttpStatusCode::from(status),
            headers: HttpHeaderMap::new(),
            body: Vec::new()
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub enum HttpStatusCode {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,
    OK = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ImUsed = 226,

    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    SwitchProxy = 306,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,

    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511
}

impl HttpStatusCode {
    pub fn code(&self) -> u16 {
        self.clone() as u16
    }

    pub fn is_informational(&self) -> bool {
        match self {
            HttpStatusCode::Continue => true,
            HttpStatusCode::SwitchingProtocols => true,
            HttpStatusCode::Processing => true,
            HttpStatusCode::EarlyHints => true,
            _ => false
        }
    }

    pub fn is_success(&self) -> bool {
        match self {
            HttpStatusCode::OK => true,
            HttpStatusCode::Created => true,
            HttpStatusCode::Accepted => true,
            HttpStatusCode::NonAuthoritativeInformation => true,
            HttpStatusCode::NoContent => true,
            HttpStatusCode::ResetContent => true,
            HttpStatusCode::PartialContent => true,
            HttpStatusCode::MultiStatus => true,
            HttpStatusCode::AlreadyReported => true,
            HttpStatusCode::ImUsed => true,
            _ => false
        }
    }

    pub fn is_redirection(&self) -> bool {
        match self {
            HttpStatusCode::MultipleChoices => true,
            HttpStatusCode::MovedPermanently => true,
            HttpStatusCode::Found => true,
            HttpStatusCode::SeeOther => true,
            HttpStatusCode::NotModified => true,
            HttpStatusCode::UseProxy => true,
            HttpStatusCode::SwitchProxy => true,
            HttpStatusCode::TemporaryRedirect => true,
            HttpStatusCode::PermanentRedirect => true,
            _ => false
        }
    }

    pub fn is_client_error(&self) -> bool {
        match self {
            HttpStatusCode::BadRequest => true,
            HttpStatusCode::Unauthorized => true,
            HttpStatusCode::PaymentRequired => true,
            HttpStatusCode::Forbidden => true,
            HttpStatusCode::NotFound => true,
            HttpStatusCode::MethodNotAllowed => true,
            HttpStatusCode::NotAcceptable => true,
            HttpStatusCode::ProxyAuthenticationRequired => true,
            HttpStatusCode::RequestTimeout => true,
            HttpStatusCode::Conflict => true,
            HttpStatusCode::Gone => true,
            HttpStatusCode::LengthRequired => true,
            HttpStatusCode::PreconditionFailed => true,
            HttpStatusCode::PayloadTooLarge => true,
            HttpStatusCode::UriTooLong => true,
            HttpStatusCode::UnsupportedMediaType => true,
            HttpStatusCode::RangeNotSatisfiable => true,
            HttpStatusCode::ExpectationFailed => true,
            HttpStatusCode::ImATeapot => true,
            HttpStatusCode::MisdirectedRequest => true,
            HttpStatusCode::UnprocessableEntity => true,
            HttpStatusCode::Locked => true,
            HttpStatusCode::FailedDependency => true,
            HttpStatusCode::TooEarly => true,
            HttpStatusCode::UpgradeRequired => true,
            HttpStatusCode::PreconditionRequired => true,
            HttpStatusCode::TooManyRequests => true,
            HttpStatusCode::RequestHeaderFieldsTooLarge => true,
            HttpStatusCode::UnavailableForLegalReasons => true,
            _ => false
        }
    }

    pub fn is_server_error(&self) -> bool {
        match self {
            HttpStatusCode::InternalServerError => true,
            HttpStatusCode::NotImplemented => true,
            HttpStatusCode::BadGateway => true,
            HttpStatusCode::ServiceUnavailable => true,
            HttpStatusCode::GatewayTimeout => true,
            HttpStatusCode::HttpVersionNotSupported => true,
            HttpStatusCode::VariantAlsoNegotiates => true,
            HttpStatusCode::InsufficientStorage => true,
            HttpStatusCode::LoopDetected => true,
            HttpStatusCode::NotExtended => true,
            HttpStatusCode::NetworkAuthenticationRequired => true,
            _ => false
        }
    }
}

impl Display for HttpStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.code(), self)
    }
}