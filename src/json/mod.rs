use async_trait::async_trait;
use crate::extractor::{ExtractorError, FromRequest};
use crate::headers;
use crate::request::{ContentType, HttpRequest};
use crate::response::into::IntoResponse;
use crate::response::{HttpResponse, HttpStatusCode};
use serde::{Deserialize, Serialize};

pub struct JsonBody<T>(pub T);

#[async_trait]
impl<T> FromRequest for JsonBody<T>
where
        for<'a> T: Deserialize<'a>
{
    async fn from_request(_index: usize, request: &mut HttpRequest) -> Result<Self, ExtractorError>
    where Self: Sized {
        if request.content_type() != Some(ContentType::ApplicationJson) {
            return Err(ExtractorError::UnexpectedContentType);
        }

        let body = request.read_body().await;
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