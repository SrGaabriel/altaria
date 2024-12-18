pub mod state;
pub mod param;
pub mod req;

use crate::request::HttpRequest;

pub trait FromRequest {
    fn from_request(index: usize, request: &HttpRequest) -> Result<Self, ExtractorError> where Self : Sized;
}

#[derive(Debug)]
pub enum ExtractorError {
    UnregisteredPath,
    WrongProvidedFormat,
    UnregisteredExtension,
    BodyParseError
}