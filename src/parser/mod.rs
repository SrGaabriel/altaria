pub mod alpha;
pub mod beta;
mod body;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum HttpParserError {
    RequestLine,
    ConnectionPreface,
    InvalidFrame,
    InvalidMethod,
    InvalidStream,
    InvalidHeader,
    InvalidRequestLine,
    FrameHeader,
    FramePayload,
    HeaderLine,
    UnknownFrameType,
    HeaderDecoding,
    RequiredHeaderNotFound,
}

impl From<hpack::decoder::DecoderError> for HttpParserError {
    fn from(_: hpack::decoder::DecoderError) -> Self {
        HttpParserError::HeaderDecoding
    }
}