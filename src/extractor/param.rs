use crate::extractor::{ExtractorError, FromRequest};
use crate::request::HttpRequest;

pub struct Param<T>(pub T);

impl<T> Param<T> {
    pub fn new(value: T) -> Self {
        Param(value)
    }
}

impl<T> FromRequest for Param<T> where T : TryFrom<String> {
    fn from_request(index: usize, request: &HttpRequest) -> Result<Self, ExtractorError>
    where
        Self: Sized
    {
        Ok(Param(T::try_from(request
            .path_values
            .as_ref()
            .ok_or::<ExtractorError>(ExtractorError::UnregisteredPath.into())
            ?.values()
            .nth(index)
            .ok_or::<ExtractorError>(ExtractorError::UnregisteredPath.into())
            ?.clone()
        ).map_err(|_| ExtractorError::WrongProvidedFormat)?))
    }
}
