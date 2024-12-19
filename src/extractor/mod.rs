pub mod state;
pub mod param;
pub mod req;
pub mod query;

use async_trait::async_trait;
use crate::request::HttpRequest;

#[async_trait]
pub trait FromRequest {
    async fn from_request(index: usize, request: &mut HttpRequest) -> Result<Self, ExtractorError> where Self : Sized;
}

#[derive(Debug)]
pub enum ExtractorError {
    UnregisteredPath,
    MissingQueryParameter,
    WrongProvidedFormat,
    UnregisteredExtension,
    BodyParseError,
    UnexpectedContentType
}