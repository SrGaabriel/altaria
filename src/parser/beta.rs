use crate::parser::HttpParserError;
use crate::request::{HttpHeader, HttpHeaderMap, HttpMethod, HttpProtocol, HttpRequest, HttpScheme};
use hpack::Decoder as HpackDecoder;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use crate::parser::body::read_body_based_on_headers;

const CONNECTION_PREFACE: &[u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
const CONNECTION_PREFACE_LEN: usize = CONNECTION_PREFACE.len();
const FRAME_HEADER_SIZE: usize = 9;
const HEADER_FLAG_ONE: u8 = 0x08;
const HEADER_FLAG_TWO: u8 = 0x20;
const FRAME_TYPE_HEADERS: u8 = 0x01;
const FRAME_TYPE_CONTINUATION: u8 = 0x04;
const FRAME_TYPE_DATA: u8 = 0x08;
const FRAME_TYPE_SETTINGS: u8 = 0x07;

pub struct BetaHttpParser<'a> {
    decoder: HpackDecoder<'a>
}

impl<'a> BetaHttpParser<'a> {
    pub fn new() -> Self {
        BetaHttpParser {
            decoder: HpackDecoder::new()
        }
    }

    pub async fn parse(&mut self, data: &mut TcpStream) -> Result<HttpRequest, HttpParserError> {
        let mut reader = BufReader::new(data);
        let mut connection_preface = vec![0; CONNECTION_PREFACE_LEN];

        reader.read_exact(&mut connection_preface).await
            .map_err(|_| HttpParserError::ConnectionPreface)?;

        let headers = self.parse_headers(&mut reader).await?;
        let method = headers.get(&HttpHeader::PseudoMethod)
            .map(|method| HttpMethod::from_str(&method))
            .ok_or(HttpParserError::RequiredHeaderNotFound)?;
        let path = headers.get(&HttpHeader::PseudoPath)
            .map(|path| path.to_string())
            .ok_or(HttpParserError::RequiredHeaderNotFound)?;
        let scheme = headers.get(&HttpHeader::PseudoScheme)
            .map(|scheme| HttpScheme::from_str(&scheme))
            .unwrap_or(HttpScheme::HTTP);

        let body = read_body_based_on_headers(&headers, &mut reader).await;

        Ok(HttpRequest {
            protocol: HttpProtocol::HTTP2,
            scheme,
            method,
            path,
            headers,
            body,
            path_values: None
        })
    }

    async fn parse_headers(&mut self, reader: &mut BufReader<&mut TcpStream>) -> Result<HttpHeaderMap, HttpParserError> {
        loop {
            let mut frame_header = [0u8; FRAME_HEADER_SIZE];
            reader.read_exact(&mut frame_header).await
                .map_err(|_| HttpParserError::FrameHeader)?;

            let length = u32::from_be_bytes([0, frame_header[0], frame_header[1], frame_header[2]]) as usize;
            let frame_type = frame_header[3];
            let flags = frame_header[4];

            let mut payload = vec![0; length];
            reader.read_exact(&mut payload).await
                .map_err(|_| HttpParserError::FramePayload)?;

            match frame_type {
                FRAME_TYPE_HEADERS => return self.parse_headers_frame(&payload, flags),
                FRAME_TYPE_CONTINUATION | FRAME_TYPE_DATA | FRAME_TYPE_SETTINGS => continue,
                _ => return Err(HttpParserError::UnknownFrameType),
            }
        }
    }

    fn parse_headers_frame(&mut self, payload: &[u8], flags: u8) -> Result<HttpHeaderMap, HttpParserError> {
        let mut cursor = 0;
        if flags & HEADER_FLAG_ONE != 0 {
            cursor += 1;
        }

        if flags & HEADER_FLAG_TWO != 0 {
            cursor += 4;
        }

        let header_data = &payload[cursor..];
        let mut headers = HttpHeaderMap::new();

        self.decoder.decode_with_cb(header_data, |name, value| {
            let name = String::from_utf8_lossy(&name);
            let value = String::from_utf8_lossy(&value);
            headers.insert(HttpHeader::from_name(&name), value.to_string());
        })?;

        Ok(headers)
    }
}

impl Clone for BetaHttpParser<'_> {
    fn clone(&self) -> Self {
        BetaHttpParser::new()
    }
}