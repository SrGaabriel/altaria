pub mod body;

use serde::{Deserialize, Serialize};
use crate::extractor::{ExtractorError, FromRequest};
use crate::headers;
use crate::request::HttpRequest;
use crate::response::{HttpResponse, HttpStatusCode};
use crate::response::into::IntoResponse;

pub struct JsonBody<T>(pub T);

impl<T> FromRequest for JsonBody<T>
where
        for<'a> T: Deserialize<'a>
{
    fn from_request(_index: usize, request: &HttpRequest) -> Result<Self, ExtractorError>
    where
        Self: Sized
    {
        let body = &request.body;
        let value = serde_json::from_slice(&*body).map_err(|_| ExtractorError::BodyParseError)?;
        Ok(JsonBody(value))
    }
}

impl<T : Serialize> IntoResponse for JsonBody<T> {
    fn into_response(self) -> HttpResponse {
        match serde_json::to_vec(&self.0) {
            Ok(body) => HttpResponse {
                status_code: HttpStatusCode::OK,
                headers: headers! {
                    ContentType: "application/json"
                },
                body
            },
            Err(e) => panic!("Failed to serialize response: {}", e)
        }
    }
}