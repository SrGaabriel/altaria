use crate::extractor::{ExtractorError, FromRequest};
use crate::request::HttpRequest;
use std::str::FromStr;
use async_trait::async_trait;

pub struct Query<T>(pub T);
pub struct OptionalQuery<T>(pub Option<T>);

impl<T> Query<T> {
    pub fn new(value: T) -> Self {
        Query(value)
    }
}

pub trait NamedExtractor {
    fn from_request_by_name(name: &str, request: &HttpRequest) -> Result<Self, ExtractorError>
    where
        Self: Sized;
}

impl<T: FromStr> NamedExtractor for Query<T> {
    fn from_request_by_name(name: &str, request: &HttpRequest) -> Result<Self, ExtractorError> {
        let query_value = request.path_values.as_ref().unwrap().queries.get(name);

        match query_value {
            Some(value) => Ok(Query(T::from_str(value).map_err(|_| ExtractorError::WrongProvidedFormat)?)),
            None => Err(ExtractorError::MissingQueryParameter),
        }
    }
}

impl<T> NamedExtractor for OptionalQuery<T> where T : FromStr {
    fn from_request_by_name(name: &str, request: &HttpRequest) -> Result<Self, ExtractorError> {
        let query_value = request.path_values.as_ref().unwrap().queries.get(name);

        match query_value {
            Some(value) => Ok(OptionalQuery(Some(T::from_str(value).map_err(|_| ExtractorError::WrongProvidedFormat)?))),
            None => Ok(OptionalQuery(None)),
        }
    }
}

#[async_trait]
impl<T> FromRequest for Query<T>
where T : FromStr {
    async fn from_request(index: usize, request: &mut HttpRequest) -> Result<Self, ExtractorError>
    where
        Self: Sized
    {
        let path_values = request
            .path_values
            .as_ref()
            .unwrap();
        Ok(Query(T::from_str(&path_values
            .queries
            .values()
            .nth(index - &path_values.params.len())
            .ok_or::<ExtractorError>(ExtractorError::MissingQueryParameter.into())
            ?.clone()
        ).map_err(|_| ExtractorError::WrongProvidedFormat)?))
    }
}