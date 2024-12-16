use crate::extractor::{ExtractorError, FromRequest};
use crate::request::HttpRequest;

impl FromRequest for HttpRequest {
    fn from_request(_index: usize, request: &HttpRequest) -> Result<Self, ExtractorError>
    where
        Self: Sized
    {
        Ok(request.clone())
    }
}