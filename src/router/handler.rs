use async_trait::async_trait;
use crate::request::HttpRequest;
use crate::response::HttpResponse;

#[async_trait]
pub trait RouteHandler {
    fn handles_method(&self, _method: crate::request::HttpMethod) -> bool {
        true
    }

    async fn handle(&self, request: HttpRequest) -> HttpResponse;
}