use thiserror::Error;
use crate::request::from::FromRequest;
use crate::request::HttpRequest;

pub struct Path<T>(pub T);

impl<T> Path<T> {
    pub fn new(value: T) -> Self {
        Path(value)
    }
}

impl<T> FromRequest for Path<T> where T : TryFrom<String> {
    fn from_request(index: usize, request: &HttpRequest) -> crate::Result<Self>
    where
        Self: Sized
    {
        Ok(Path(T::try_from(request
            .path_values
            .as_ref()
            .ok_or::<ExtractorError>(ExtractorError::NotFound.into())
            ?.values()
            .nth(index)
            .ok_or::<ExtractorError>(ExtractorError::NotFound.into())
            ?.clone()
        ).map_err(|_| ExtractorError::WrongFormat)?))
    }
}

#[derive(Debug, Error)]
pub enum ExtractorError {
    #[error("Value not found")]
    NotFound,
    #[error("Value is in the wrong format")]
    WrongFormat
}