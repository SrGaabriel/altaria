use crate::request::HttpRequest;

pub trait FromRequest {
    fn from_request(index: usize, request: &HttpRequest) -> crate::Result<Self> where Self : Sized;
}