use crate::parser::{HttpParser, HttpParserError};
use crate::request::{HttpHeader, HttpHeaderMap, HttpMethod, HttpRequest};
use std::collections::HashMap;
use async_trait::async_trait;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::TcpStream;

pub struct AlphaHttpParser {

}

impl AlphaHttpParser {
    async fn parse_request_line(&self, mut reader: &mut BufReader<&mut TcpStream>) -> Result<HttpMethod, HttpParserError> {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).await;
        if bytes_read.is_err() {
            return Err(HttpParserError { message: "Failed to read request line".to_string() });
        }

        let bytes = line.as_bytes();
        let mut end = line.len();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                end = i;
                break;
            }
        }

        let method_str = &line[..end];
        let method = match method_str {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "OPTIONS" => HttpMethod::OPTIONS,
            "HEAD" => HttpMethod::HEAD,
            "PATCH" => HttpMethod::PATCH,
            "TRACE" => HttpMethod::TRACE,
            _ => return Err(HttpParserError { message: format!("Invalid method: {}", method_str) })
        };

        Ok(method)
    }

    async fn parse_headers(&self, reader: &mut BufReader<&mut TcpStream>) -> Result<HttpHeaderMap, HttpParserError> {
        let mut headers = HashMap::new();

        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).await;
            if bytes_read.is_err() {
                return Err(HttpParserError { message: "Failed to read header line".to_string() });
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
}

#[async_trait]
impl HttpParser for AlphaHttpParser {
    async fn parse(&self, data: &mut TcpStream) -> Result<HttpRequest, HttpParserError> {
        let mut buffer = BufReader::new(data);
        let method = self.parse_request_line(&mut buffer).await?;
        let headers = self.parse_headers(&mut buffer).await?;

        let content_length = headers.get(&HttpHeader::ContentLength);

        let body = if let Some(content_length) = content_length {
            let content_length = content_length.parse::<usize>().unwrap();
            let mut limited_reader = buffer.take(content_length as u64);
            let mut body = Vec::new();
            limited_reader.read_to_end(&mut body).await.unwrap();
            body
        } else {
            Vec::new()
        };

        Ok(HttpRequest {
            method,
            headers,
            body
        })
    }
}

unsafe impl Send for AlphaHttpParser {}
unsafe impl Sync for AlphaHttpParser {}