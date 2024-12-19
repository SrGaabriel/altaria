pub mod alpha;
pub mod body;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum HttpParserError {
    RequestLine,
    ConnectionPreface,
    InvalidFrame,
    InvalidMethod,
    InvalidStream,
    InvalidHeader,
    InvalidContentLength,
    InvalidRequestLine,
    FrameHeader,
    FramePayload,
    HeaderLine,
    UnknownFrameType,
    HeaderDecoding,
    RequiredHeaderNotFound,
}