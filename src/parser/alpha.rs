use std::collections::HashMap;
use std::io::{Bytes, Cursor, Read};
use crate::parser::{HttpParser, HttpParserError};
use crate::request::{HttpHeader, HttpHeaderMap, HttpMethod, HttpRequest};

pub struct AlphaHttpParser {

}

impl AlphaHttpParser {
    fn parse_request_line(&self, line: &str) -> Result<HttpMethod, HttpParserError> {
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

    fn parse_headers(&self, lines: &Vec<&str>) -> Result<HttpHeaderMap, HttpParserError> {
        // Again, we'll do the most efficient parsing possible
        let mut headers = HashMap::new();
        for line in lines.iter().skip(1) {
            if line.is_empty() {
                break;
            }

            let mut end = line.len();
            for (j, &item) in line.as_bytes().iter().enumerate() {
                if item == b':' {
                    end = j;
                    break;
                }
            }
            if end == line.len() {
                return Err(HttpParserError { message: format!("Invalid header: {}", line) });
            }

            let header_name = &line[..end];
            let header_value = &line[end + 2..];
            let header = HttpHeader::from_name(header_name);

            headers.insert(header, header_value.to_string());
        }

        Ok(headers)
    }
}

impl HttpParser for AlphaHttpParser {
    fn parse(&self, data: &mut dyn Read) -> Result<HttpRequest, HttpParserError> {
        let mut buf = [0; 1024];
        let string = match data.read(&mut buf) {
            Ok(bytes_read) => {
                String::from_utf8_lossy(&buf[..bytes_read])
            }
            Err(err) => {
                return Err(HttpParserError { message: format!("Failed to read from stream: {}", err) });
            }
        };

        let lines = string.lines().collect::<Vec<&str>>();
        let method = self.parse_request_line(lines[0])?;
        let headers = self.parse_headers(&lines)?;

        let body = lines.iter().skip(1 + headers.len()).fold(Vec::new(), |mut acc, &line| {
            acc.extend_from_slice(line.as_bytes());
            acc
        });

        Ok(HttpRequest {
            method,
            headers,
            body
        })
    }
}

unsafe impl Send for AlphaHttpParser {}
unsafe impl Sync for AlphaHttpParser {}