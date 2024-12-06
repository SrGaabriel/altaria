use async_trait::async_trait;
use crate::request::HttpRequest;
use crate::response::HttpResponse;

#[async_trait]
pub trait RouteHandler {
    async fn handle(&self, request: HttpRequest) -> HttpResponse;
}