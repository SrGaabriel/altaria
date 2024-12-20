use std::str::FromStr;
use async_trait::async_trait;
use crate::extractor::{ExtractorError, FromRequest};
use crate::request::HttpRequest;

pub struct Param<T>(pub T);

impl<T> Param<T> {
    pub fn new(value: T) -> Self {
        Param(value)
    }
}

#[async_trait]
impl<T> FromRequest for Param<T> where T : FromStr {
    async fn from_request(index: usize, request: &mut HttpRequest) -> Result<Self, ExtractorError>
    where
        Self: Sized
    {
        Ok(Param(T::from_str(&request
            .path_values
            .get()
            .ok_or::<ExtractorError>(ExtractorError::UnregisteredPath.into())
            ?.params
            .values()
            .nth(index)
            .ok_or::<ExtractorError>(ExtractorError::UnregisteredPath.into())
            ?.clone()
        ).map_err(|_| ExtractorError::WrongProvidedFormat)?))
    }
}
