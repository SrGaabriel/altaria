use std::io::Read;
use async_trait::async_trait;
use tokio::net::TcpStream;
use crate::request::HttpRequest;

pub mod alpha;

#[async_trait]
pub trait HttpParser {
    async fn parse(&self, data: &mut TcpStream) -> Result<HttpRequest, HttpParserError>;
}

#[derive(Debug, Clone)]
pub struct HttpParserError {
    pub message: String
}

impl HttpParserError {
    pub fn new(message: &str) -> HttpParserError {
        HttpParserError { message: message.to_string() }
    }
}