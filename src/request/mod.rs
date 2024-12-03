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
    MaxForwards,
    Origin,
    Pragma,
    Prefer,
    ProxyAuthorization,
    Range,
    Referer,
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
    pub(crate) fn from_name(name: &str) -> HttpHeader {
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
            "Max-Forwards" => HttpHeader::MaxForwards,
            "Origin" => HttpHeader::Origin,
            "Pragma" => HttpHeader::Pragma,
            "Prefer" => HttpHeader::Prefer,
            "Proxy-Authorization" => HttpHeader::ProxyAuthorization,
            "Range" => HttpHeader::Range,
            "Referer" => HttpHeader::Referer,
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