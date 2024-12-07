use crate::request::from::FromRequest;
use crate::request::HttpRequest;
use crate::response::into::IntoResponse;
use crate::response::HttpResponse;
use crate::router::handler::RouteHandler;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Clone)]
pub struct CallbackRouteHandler {
    func: Arc<dyn Fn(HttpRequest) -> Pin<Box<dyn Future<Output = HttpResponse> + Send>> + Send + Sync>
}

#[async_trait]
impl RouteHandler for CallbackRouteHandler {
    async fn handle(&self, request: HttpRequest) -> HttpResponse {
        (self.func)(request).await
    }
}

pub fn function_handler<F, Fut, R>(callback: F) -> CallbackRouteHandler
where F : Fn(HttpRequest) -> Fut + Send + Sync + 'static + Clone,
      Fut : Future<Output = R> + Send + 'static,
      R : IntoResponse + Send + 'static
{
    CallbackRouteHandler {
        func: Arc::new(move |request| Box::pin({
            let value = callback.clone();
            async move {
                value.clone()(request).await.into_response()
            }
        }))
    }
}

#[async_trait]
impl<F, Fut, R> RouteHandler for F
where
    F : Fn(HttpRequest) -> Fut + Send + Sync + 'static + Clone,
    Fut : Future<Output = R> + Send + 'static,
    R : IntoResponse + Send + 'static,
{
    async fn handle(&self, request: HttpRequest) -> HttpResponse {
        self(request).await.into_response()
    }
}

#[async_trait]
impl FunctionRouteHandler<()> for CallbackRouteHandler {
    async fn handle_request(&self, request: HttpRequest) -> HttpResponse {
        (self.func)(request).await
    }
}

#[async_trait]
impl<F, Fut, R> FunctionRouteHandler<()> for F
where
    F : (Fn(HttpRequest) -> Fut) + Send + Sync + 'static + Clone,
    Fut : Future<Output = R> + Send + 'static,
    R : IntoResponse + Send + 'static
{
    async fn handle_request(&self, request: HttpRequest) -> HttpResponse {
        self(request).await.into_response()
    }
}

#[async_trait]
impl<F, Fut, R, E1> FunctionRouteHandler<(E1)> for F
where
    F : (Fn(E1, HttpRequest) -> Fut) + Send + Sync + 'static + Clone,
    Fut : Future<Output = R> + Send + 'static,
    R : IntoResponse + Send + 'static,
    E1 : FromRequest,
{
    async fn handle_request(&self, request: HttpRequest) -> HttpResponse {
        let e1value = E1::from_request(0, &request).expect("Failed to extract value");
        self(e1value, request).await.into_response()
    }
}

#[async_trait]
pub trait FunctionRouteHandler<Extractors> : Sync {
    async fn handle_request(&self, request: HttpRequest) -> HttpResponse;

    fn into_route_handler(self) -> CallbackRouteHandler where Self: Sized + Send + 'static {
        let pointer_to_self = Arc::new(self);
        CallbackRouteHandler {
            func: Arc::new(move |request| Box::pin({
                let self_value = pointer_to_self.clone();
                async move {
                    self_value.handle_request(request).await
                }
            }))
        }
    }
}

pub struct RouteHandlerPhantomType;

pub trait IntoRouteHandler {
    fn into_route_handler(self) -> CallbackRouteHandler;
}
