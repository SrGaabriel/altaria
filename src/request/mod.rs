use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub headers: HttpHeaderMap,
    pub body: Vec<u8>
}

pub type HttpHeaderMap = HashMap<HttpHeader, String>;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    HEAD,
    PATCH,
    TRACE
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub enum HttpHeader {
    AIM,
    Accept,
    AcceptCharset,
    AcceptDatetime,
    AcceptEncoding,
    AcceptLanguage,
    AccessControlRequestMethod,
    Authorization,
    CacheControl,
    Connection,
    ContentEncoding,
    ContentLength,
    ContentMd5,
    ContentType,
    Cookie,
    Date,
    Expect,
    Forwarded,
    From,
    Host,
    Http2Settings,
    IfMatch,
    IfModifiedSince,
    IfNoneMatch,
    IfRange,
    IfUnmodifiedSince,
    Location,
    MaxForwards,
    Origin,
    Pragma,
    Prefer,
    ProxyAuthorization,
    Range,
    Referer,
    Server,
    Te,
    Trailer,
    TransferEncoding,
    UserAgent,
    Upgrade,
    Via,
    Warning,
    Custom(String)
}

impl HttpHeader {
    pub fn name(&self) -> String {
        match self {
            HttpHeader::AIM => "AIM".to_string(),
            HttpHeader::Accept => "Accept".to_string(),
            HttpHeader::AcceptCharset => "Accept-Charset".to_string(),
            HttpHeader::AcceptDatetime => "Accept-Datetime".to_string(),
            HttpHeader::AcceptEncoding => "Accept-Encoding".to_string(),
            HttpHeader::AcceptLanguage => "Accept-Language".to_string(),
            HttpHeader::AccessControlRequestMethod => "Access-Control-Request-Method".to_string(),
            HttpHeader::Authorization => "Authorization".to_string(),
            HttpHeader::CacheControl => "Cache-Control".to_string(),
            HttpHeader::Connection => "Connection".to_string(),
            HttpHeader::ContentEncoding => "Content-Encoding".to_string(),
            HttpHeader::ContentLength => "Content-Length".to_string(),
            HttpHeader::ContentMd5 => "Content-MD5".to_string(),
            HttpHeader::ContentType => "Content-Type".to_string(),
            HttpHeader::Cookie => "Cookie".to_string(),
            HttpHeader::Date => "Date".to_string(),
            HttpHeader::Expect => "Expect".to_string(),
            HttpHeader::Forwarded => "Forwarded".to_string(),
            HttpHeader::From => "From".to_string(),
            HttpHeader::Host => "Host".to_string(),
            HttpHeader::Http2Settings => "HTTP2-Settings".to_string(),
            HttpHeader::IfMatch => "If-Match".to_string(),
            HttpHeader::IfModifiedSince => "If-Modified-Since".to_string(),
            HttpHeader::IfNoneMatch => "If-None-Match".to_string(),
            HttpHeader::IfRange => "If-Range".to_string(),
            HttpHeader::IfUnmodifiedSince => "If-Unmodified-Since".to_string(),
            HttpHeader::Location => "Location".to_string(),
            HttpHeader::MaxForwards => "Max-Forwards".to_string(),
            HttpHeader::Origin => "Origin".to_string(),
            HttpHeader::Pragma => "Pragma".to_string(),
            HttpHeader::Prefer => "Prefer".to_string(),
            HttpHeader::ProxyAuthorization => "Proxy-Authorization".to_string(),
            HttpHeader::Range => "Range".to_string(),
            HttpHeader::Referer => "Referer".to_string(),
            HttpHeader::Server => "Server".to_string(),
            HttpHeader::Te => "TE".to_string(),
            HttpHeader::Trailer => "Trailer".to_string(),
            HttpHeader::TransferEncoding => "Transfer-Encoding".to_string(),
            HttpHeader::UserAgent => "User-Agent".to_string(),
            HttpHeader::Upgrade => "Upgrade".to_string(),
            HttpHeader::Via => "Via".to_string(),
            HttpHeader::Warning => "Warning".to_string(),
            HttpHeader::Custom(name) => name.to_string()
        }
    }

    pub fn from_name(name: &str) -> HttpHeader {
        match name {
            "AIM" => HttpHeader::AIM,
            "Accept" => HttpHeader::Accept,
            "Accept-Charset" => HttpHeader::AcceptCharset,
            "Accept-Datetime" => HttpHeader::AcceptDatetime,
            "Accept-Encoding" => HttpHeader::AcceptEncoding,
            "Accept-Language" => HttpHeader::AcceptLanguage,
            "Access-Control-Request-Method" => HttpHeader::AccessControlRequestMethod,
            "Authorization" => HttpHeader::Authorization,
            "Cache-Control" => HttpHeader::CacheControl,
            "Connection" => HttpHeader::Connection,
            "Content-Encoding" => HttpHeader::ContentEncoding,
            "Content-Length" => HttpHeader::ContentLength,
            "Content-MD5" => HttpHeader::ContentMd5,
            "Content-Type" => HttpHeader::ContentType,
            "Cookie" => HttpHeader::Cookie,
            "Date" => HttpHeader::Date,
            "Expect" => HttpHeader::Expect,
            "Forwarded" => HttpHeader::Forwarded,
            "From" => HttpHeader::From,
            "Host" => HttpHeader::Host,
            "HTTP2-Settings" => HttpHeader::Http2Settings,
            "If-Match" => HttpHeader::IfMatch,
            "If-Modified-Since" => HttpHeader::IfModifiedSince,
            "If-None-Match" => HttpHeader::IfNoneMatch,
            "If-Range" => HttpHeader::IfRange,
            "If-Unmodified-Since" => HttpHeader::IfUnmodifiedSince,
            "Location" => HttpHeader::Location,
            "Max-Forwards" => HttpHeader::MaxForwards,
            "Origin" => HttpHeader::Origin,
            "Pragma" => HttpHeader::Pragma,
            "Prefer" => HttpHeader::Prefer,
            "Proxy-Authorization" => HttpHeader::ProxyAuthorization,
            "Range" => HttpHeader::Range,
            "Referer" => HttpHeader::Referer,
            "Server" => HttpHeader::Server,
            "TE" => HttpHeader::Te,
            "Trailer" => HttpHeader::Trailer,
            "Transfer-Encoding" => HttpHeader::TransferEncoding,
            "User-Agent" => HttpHeader::UserAgent,
            "Upgrade" => HttpHeader::Upgrade,
            "Via" => HttpHeader::Via,
            "Warning" => HttpHeader::Warning,
            _ => HttpHeader::Custom(name.to_string())
        }
    }
}

#[macro_export]
macro_rules! headers {
    ($($key:ident: $value:expr),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert(crate::request::HttpHeader::$key, $value.to_string());
            )*
            map
        }
    };
}