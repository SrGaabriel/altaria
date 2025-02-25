use crate::parser::body::LazyBodyReader;
use crate::parser::HttpParserError;
use crate::request::{HttpHeader, HttpHeaderMap, HttpMethod, HttpProtocol, HttpRequest, HttpScheme};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;
use tokio::sync::OnceCell;

const INITIAL_BUFFER_SIZE: usize = 4096;
const MAX_HEADER_SIZE: usize = 8192;

pub struct AlphaHttpParser {
    method_map: HashMap<&'static str, HttpMethod>
}

impl AlphaHttpParser {
    pub fn new() -> Self {
        let mut method_map = HashMap::with_capacity(8);
        method_map.insert("GET", HttpMethod::GET);
        method_map.insert("POST", HttpMethod::POST);
        method_map.insert("PUT", HttpMethod::PUT);
        method_map.insert("DELETE", HttpMethod::DELETE);
        method_map.insert("OPTIONS", HttpMethod::OPTIONS);
        method_map.insert("HEAD", HttpMethod::HEAD);
        method_map.insert("PATCH", HttpMethod::PATCH);
        method_map.insert("TRACE", HttpMethod::TRACE);

        AlphaHttpParser { method_map }
    }

    async fn parse_request_line(&self, reader: &mut BufReader<OwnedReadHalf>) -> Result<(HttpMethod, String), HttpParserError> {
        let mut line = Vec::with_capacity(INITIAL_BUFFER_SIZE);
        let bytes_read = reader.read_until(b'\n', &mut line).await.map_err(|_| HttpParserError::RequestLine)?;

        if bytes_read < 5 {
            return Err(HttpParserError::InvalidRequestLine);
        }

        let line_str = match std::str::from_utf8(&line) {
            Ok(s) => s,
            Err(_) => return Err(HttpParserError::InvalidRequestLine),
        };

        let mut parts = line_str.split_whitespace();
        let method_str = parts.next().ok_or(HttpParserError::InvalidRequestLine)?.trim_end();
        let path = parts.next().ok_or(HttpParserError::InvalidRequestLine)?.trim_end().to_string();

        let method = self.method_map.get(method_str).ok_or(HttpParserError::InvalidMethod)?;

        Ok((*method, path))
    }

    async fn parse_headers(&self, reader: &mut BufReader<OwnedReadHalf>) -> Result<HttpHeaderMap, HttpParserError> {
        let mut headers = HttpHeaderMap::with_capacity(16);
        let mut buffer = Vec::with_capacity(INITIAL_BUFFER_SIZE);

        loop {
            buffer.clear();
            let bytes_read = reader.read_until(b'\n', &mut buffer).await.map_err(|_| HttpParserError::HeaderLine)?;

            if bytes_read == 0 || buffer.len() <= 2 {
                break;
            }

            if buffer.len() > MAX_HEADER_SIZE {
                return Err(HttpParserError::HeaderLine);
            }

            if let Some(idx) = buffer.iter().position(|&b| b == b':') {
                let key = std::str::from_utf8(&buffer[..idx]).map_err(|_| HttpParserError::HeaderLine)?;
                let value = std::str::from_utf8(&buffer[idx + 1..]).map_err(|_| HttpParserError::HeaderLine)?;

                headers.insert(HttpHeader::from_name(&key.trim().to_lowercase()), value.trim().to_string());
            }
        }

        Ok(headers)
    }

    pub(crate) async fn parse(&self, addr: SocketAddr, stream: OwnedReadHalf) -> Result<HttpRequest, HttpParserError> {
        let mut reader = BufReader::with_capacity(INITIAL_BUFFER_SIZE, stream);
        let (method, path) = self.parse_request_line(&mut reader).await?;
        let headers = self.parse_headers(&mut reader).await?;

        let content_length = headers
            .get(&HttpHeader::ContentLength)
            .ok_or(HttpParserError::InvalidContentLength)?
            .parse::<usize>()
            .map_err(|_| HttpParserError::InvalidContentLength)?;

        let body_reader = LazyBodyReader::new(reader, content_length);

        Ok(HttpRequest {
            protocol: HttpProtocol::HTTP1,
            path,
            scheme: HttpScheme::HTTP,
            method,
            headers,
            body_reader,
            content_length,
            peer_addr: addr,
            flow: OnceCell::new(),
            path_values: OnceCell::new(),
        })
    }
}