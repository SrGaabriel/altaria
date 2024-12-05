use crate::parser::HttpParserError;
use crate::request::{HttpHeader, HttpHeaderMap, HttpMethod, HttpProtocol, HttpRequest, HttpScheme};
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use crate::parser::body::read_body_based_on_headers;

pub struct AlphaHttpParser {

}

impl AlphaHttpParser {
    pub fn new() -> Self {
        AlphaHttpParser {}
    }

    async fn parse_request_line(&self, mut reader: &mut BufReader<&mut TcpStream>) -> Result<(HttpMethod, String), HttpParserError> {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).await;

        if bytes_read.is_err() {
            return Err(HttpParserError::RequestLine);
        }

        let line_parts = line.split_whitespace().collect::<Vec<&str>>();
        let method_str = *line_parts.get(0).ok_or(HttpParserError::InvalidRequestLine)?;
        let path = line_parts.get(1).ok_or(HttpParserError::InvalidRequestLine)?.to_string();

        let method = match method_str {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "OPTIONS" => HttpMethod::OPTIONS,
            "HEAD" => HttpMethod::HEAD,
            "PATCH" => HttpMethod::PATCH,
            "TRACE" => HttpMethod::TRACE,
            _ => return Err(HttpParserError::InvalidMethod)
        };

        Ok((method, path))
    }

    async fn parse_headers(&self, reader: &mut BufReader<&mut TcpStream>) -> Result<HttpHeaderMap, HttpParserError> {
        let mut headers = HashMap::new();

        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).await;
            if bytes_read.is_err() {
                return Err(HttpParserError::HeaderLine);
            }
            let bytes_read = bytes_read.unwrap();

            if line.trim().is_empty() || bytes_read == 0 {
                break;
            }

            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim().to_lowercase();
                let value = value.trim().to_string();
                headers.insert(HttpHeader::from_name(&key), value);
            }
        }
        Ok(headers)
    }

    pub(crate) async fn parse(&self, data: &mut TcpStream) -> Result<HttpRequest, HttpParserError> {
        let mut reader = BufReader::new(data);
        let (method, path) = self.parse_request_line(&mut reader).await?;
        let headers = self.parse_headers(&mut reader).await?;
        let body = read_body_based_on_headers(&headers, &mut reader).await;

        Ok(HttpRequest {
            protocol: HttpProtocol::HTTP1,
            path,
            scheme: HttpScheme::HTTP,
            method,
            headers,
            body
        })
    }
}

unsafe impl Send for AlphaHttpParser {}
unsafe impl Sync for AlphaHttpParser {}