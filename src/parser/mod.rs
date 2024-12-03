use std::io::Read;
use crate::request::HttpRequest;

pub mod alpha;

pub trait HttpParser {
    fn parse(&self, data: &mut dyn Read) -> Result<HttpRequest, HttpParserError>;
}

#[derive(Debug, Clone)]
pub struct HttpParserError {
    pub message: String
}