// use async_trait::async_trait;
// use crate::extractor::{ExtractorError, FromRequest};
// use crate::request::HttpRequest;
//
// #[async_trait]
// impl FromRequest for HttpRequest {
//     async fn from_request(index: usize, request: &mut HttpRequest) -> Result<Self, ExtractorError>
//     where
//         Self: Sized
//     {
//         panic!("A request should not be partially extracted from a request")
//     }
// }