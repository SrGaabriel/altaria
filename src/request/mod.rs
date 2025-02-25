use std::collections::HashMap;
use std::fmt::Display;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::OnceCell;
use crate::router::flow::RequestFlow;
use crate::parser::body::LazyBodyReader;

pub struct HttpRequest {
    pub protocol: HttpProtocol,
    pub scheme: HttpScheme,
    pub path: String,
    pub method: HttpMethod,
    pub headers: HttpHeaderMap,
    pub flow: OnceCell<Arc<RequestFlow>>,
    pub peer_addr: SocketAddr,
    pub content_length: usize,
    pub body_reader: LazyBodyReader,
    pub(crate) path_values: OnceCell<RoutePathValues>
}

unsafe impl Send for HttpRequest {}

impl HttpRequest {
    pub async fn read_body(&mut self) -> &[u8] {
        self.body_reader.read_all().await
    }

    pub fn content_type(&self) -> Option<ContentType> {
        match self.headers.get(&HttpHeader::ContentType) {
            Some(content_type) => Some(ContentType::from_str(content_type)),
            None => None
        }
    }

    pub(crate) fn set_route_path(&mut self, values: RoutePathValues) {
        self.path_values.set(values).unwrap();
    }

    pub(crate) fn set_flow(&mut self, flow: Arc<RequestFlow>) {
        let result = self.flow.set(flow);
        if let Err(_) = result {
            panic!("Flow already set for request");
        }
    }
}

pub type HttpHeaderMap = HashMap<HttpHeader, String>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum HttpProtocol {
    HTTP1 = 1,
    HTTP2 = 2
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum HttpScheme {
    HTTP,
    HTTPS
}

impl HttpScheme {
    pub fn from_str(scheme: &str) -> HttpScheme {
        match scheme {
            "http" => HttpScheme::HTTP,
            "https" => HttpScheme::HTTPS,
            _ => HttpScheme::HTTP
        }
    }
}

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

impl HttpMethod {
    pub fn from_str(method: &str) -> Option<HttpMethod> {
        match method {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            "PUT" => Some(HttpMethod::PUT),
            "DELETE" => Some(HttpMethod::DELETE),
            "OPTIONS" => Some(HttpMethod::OPTIONS),
            "HEAD" => Some(HttpMethod::HEAD),
            "PATCH" => Some(HttpMethod::PATCH),
            "TRACE" => Some(HttpMethod::TRACE),
            _ => None
        }
    }
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
    Custom(String),

    PseudoScheme,
    PseudoMethod,
    PseudoAuthority,
    PseudoPath,
    PseudoStatus,
    PseudoProtocol
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
            HttpHeader::Custom(name) => name.to_string(),

            HttpHeader::PseudoScheme => ":scheme".to_string(),
            HttpHeader::PseudoMethod => ":method".to_string(),
            HttpHeader::PseudoAuthority => ":authority".to_string(),
            HttpHeader::PseudoPath => ":path".to_string(),
            HttpHeader::PseudoStatus => ":status".to_string(),
            HttpHeader::PseudoProtocol => ":protocol".to_string()
        }
    }

    pub fn from_name(name: &str) -> HttpHeader {
        match name.to_lowercase().as_str() {
            "aim" => HttpHeader::AIM,
            "accept" => HttpHeader::Accept,
            "accept-charset" => HttpHeader::AcceptCharset,
            "accept-datetime" => HttpHeader::AcceptDatetime,
            "accept-encoding" => HttpHeader::AcceptEncoding,
            "accept-language" => HttpHeader::AcceptLanguage,
            "access-control-request-method" => HttpHeader::AccessControlRequestMethod,
            "authorization" => HttpHeader::Authorization,
            "cache-control" => HttpHeader::CacheControl,
            "connection" => HttpHeader::Connection,
            "content-encoding" => HttpHeader::ContentEncoding,
            "content-length" => HttpHeader::ContentLength,
            "content-md5" => HttpHeader::ContentMd5,
            "content-type" => HttpHeader::ContentType,
            "cookie" => HttpHeader::Cookie,
            "date" => HttpHeader::Date,
            "expect" => HttpHeader::Expect,
            "forwarded" => HttpHeader::Forwarded,
            "from" => HttpHeader::From,
            "host" => HttpHeader::Host,
            "http2-settings" => HttpHeader::Http2Settings,
            "if-match" => HttpHeader::IfMatch,
            "if-modified-since" => HttpHeader::IfModifiedSince,
            "if-none-match" => HttpHeader::IfNoneMatch,
            "if-range" => HttpHeader::IfRange,
            "if-unmodified-since" => HttpHeader::IfUnmodifiedSince,
            "location" => HttpHeader::Location,
            "max-forwards" => HttpHeader::MaxForwards,
            "origin" => HttpHeader::Origin,
            "pragma" => HttpHeader::Pragma,
            "prefer" => HttpHeader::Prefer,
            "proxy-authorization" => HttpHeader::ProxyAuthorization,
            "range" => HttpHeader::Range,
            "referer" => HttpHeader::Referer,
            "server" => HttpHeader::Server,
            "te" => HttpHeader::Te,
            "trailer" => HttpHeader::Trailer,
            "transfer-encoding" => HttpHeader::TransferEncoding,
            "user-agent" => HttpHeader::UserAgent,
            "upgrade" => HttpHeader::Upgrade,
            "via" => HttpHeader::Via,
            "warning" => HttpHeader::Warning,
            ":scheme" => HttpHeader::PseudoScheme,
            ":method" => HttpHeader::PseudoMethod,
            ":authority" => HttpHeader::PseudoAuthority,
            ":path" => HttpHeader::PseudoPath,
            ":status" => HttpHeader::PseudoStatus,
            ":protocol" => HttpHeader::PseudoProtocol,
            _ => HttpHeader::Custom(name.to_string()),
        }
    }

    pub fn is_pseudo(&self) -> bool {
        match self {
            HttpHeader::PseudoScheme | HttpHeader::PseudoMethod | HttpHeader::PseudoAuthority | HttpHeader::PseudoPath | HttpHeader::PseudoStatus | HttpHeader::PseudoProtocol => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ContentType {
    TextPlain,
    TextHtml,
    TextXml,
    ApplicationJson,
    ApplicationXml,
    ApplicationOctetStream,
    MultipartFormData,
    Custom(String)
}

impl ContentType {
    pub fn from_str(content_type: &str) -> ContentType {
        match content_type {
            "text/plain" => ContentType::TextPlain,
            "text/html" => ContentType::TextHtml,
            "text/xml" => ContentType::TextXml,
            "application/json" => ContentType::ApplicationJson,
            "application/xml" => ContentType::ApplicationXml,
            "application/octet-stream" => ContentType::ApplicationOctetStream,
            "multipart/form-data" => ContentType::MultipartFormData,
            _ => ContentType::Custom(content_type.to_string())
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            ContentType::TextPlain => "text/plain".to_string(),
            ContentType::TextHtml => "text/html".to_string(),
            ContentType::TextXml => "text/xml".to_string(),
            ContentType::ApplicationJson => "application/json".to_string(),
            ContentType::ApplicationXml => "application/xml".to_string(),
            ContentType::ApplicationOctetStream => "application/octet-stream".to_string(),
            ContentType::MultipartFormData => "multipart/form-data".to_string(),
            ContentType::Custom(content_type) => content_type.to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoutePathValues {
    pub params: HashMap<String, String>,
    pub queries: HashMap<String, String>
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::TRACE => "TRACE"
        })
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