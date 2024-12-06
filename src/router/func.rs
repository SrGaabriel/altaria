use std::future::Future;
use std::pin::Pin;
use async_trait::async_trait;
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use crate::router::handler::RouteHandler;

pub struct FunctionRouteHandler {
    func: Box<dyn Fn(HttpRequest) -> Pin<Box<dyn Future<Output = HttpResponse> + Send>> + Send + Sync>
}

#[async_trait]
impl RouteHandler for FunctionRouteHandler {
    async fn handle(&self, request: HttpRequest) -> HttpResponse {
        let handle = (self.func)(request);
        handle.await
    }
}

pub fn function_handler<F, Fut>(callback: F) -> Box<FunctionRouteHandler>
    where F : Fn(HttpRequest) -> Fut + Send + Sync + 'static,
        Fut : Future<Output = HttpResponse> + Send + 'static
{
    Box::new(FunctionRouteHandler {
        func: Box::new(move |request| Box::pin(callback(request)))
    })
}