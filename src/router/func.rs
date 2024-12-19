use crate::request::{HttpMethod, HttpRequest};
use crate::response::into::IntoResponse;
use crate::response::{HttpResponse, HttpStatusCode};
use crate::router::handler::RouteHandler;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use crate::extractor::{ExtractorError, FromRequest};

#[derive(Clone)]
pub struct CallbackRouteHandler {
    method: Option<HttpMethod>,
    func: Arc<dyn Fn(HttpRequest) -> Pin<Box<dyn Future<Output = HttpResponse> + Send>> + Send + Sync>
}

#[async_trait]
impl RouteHandler for CallbackRouteHandler {
    fn handles_method(&self, method: HttpMethod) -> bool {
        match &self.method {
            Some(m) => *m == method,
            None => true
        }
    }

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
        method: None,
        func: Arc::new(move |request| Box::pin({
            let value = callback.clone();
            async move {
                value.clone()(request).await.into_response()
            }
        }))
    }
}


#[async_trait]
pub trait FunctionRouteHandler<Extractors> : Sync {
    async fn handle_request(&self, request: HttpRequest) -> HttpResponse;

    fn get_method(&self) -> Option<HttpMethod> {
        None
    }

    fn into_route_handler(self) -> CallbackRouteHandler where Self: Sized + Send + 'static {
        let pointer_to_self = Arc::new(self);
        CallbackRouteHandler {
            method: pointer_to_self.get_method(),
            func: Arc::new(move |request| Box::pin({
                let self_value = pointer_to_self.clone();
                async move {
                    self_value.handle_request(request).await
                }
            }))
        }
    }
}

pub fn handle_function_failure(err: ExtractorError) -> HttpResponse {
    match err {
        ExtractorError::UnregisteredExtension => {
            (HttpStatusCode::InternalServerError, "altaria: This route handler expected an extension with a type that wasn't registered in the router declaration").into_response()
        }
        ExtractorError::UnregisteredPath => {
            (HttpStatusCode::InternalServerError, "altaria: This route handler expected a path value that wasn't registered in the router declaration").into_response()
        }
        ExtractorError::WrongProvidedFormat => {
            (HttpStatusCode::BadRequest, "The provided value could not be parsed").into_response()
        }
        ExtractorError::BodyParseError => {
            (HttpStatusCode::BadRequest, "The body of the request could not be parsed").into_response()
        },
        ExtractorError::UnexpectedContentType => {
            (HttpStatusCode::BadRequest, "The request did not have the expected content type").into_response()
        },
        ExtractorError::MissingQueryParameter => {
            (HttpStatusCode::BadRequest, "The request did not have the expected query parameter").into_response()
        }
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
    F : (Fn() -> Fut) + Send + Sync + 'static + Clone,
    Fut : Future<Output = R> + Send + 'static,
    R : IntoResponse + Send + 'static,
{
    async fn handle_request(&self, _request: HttpRequest) -> HttpResponse {
        self().await.into_response()
    }
}

#[async_trait]
impl<F, Fut, R, E1> FunctionRouteHandler<E1> for F
where
    F : (Fn(E1) -> Fut) + Send + Sync + 'static + Clone,
    Fut : Future<Output = R> + Send + 'static,
    R : IntoResponse + Send + 'static,
    E1 : FromRequest + Send
{
    async fn handle_request(&self, request: HttpRequest) -> HttpResponse {
        match E1::from_request(0, &request) {
            Ok(e1value) => self(e1value).await.into_response(),
            Err(err) => handle_function_failure(err)
        }
    }
}

#[async_trait]
impl<F, Fut, R, E1, E2> FunctionRouteHandler<(E1, E2)> for F
where
    F : (Fn(E1, E2) -> Fut) + Send + Sync + 'static + Clone,
    Fut : Future<Output = R> + Send + 'static,
    R : IntoResponse + Send + 'static,

    E1 : FromRequest + Send,
    E2 : FromRequest + Send,
{
    async fn handle_request(&self, request: HttpRequest) -> HttpResponse {
        let extract_values = || -> Result<(E1, E2), ExtractorError> {
            let e1value = E1::from_request(0, &request)?;
            let e2value = E2::from_request(1, &request)?;
            Ok((e1value, e2value))
        };

        match extract_values() {
            Ok((e1value, e2value)) => self(e1value, e2value).await.into_response(),
            Err(err) => handle_function_failure(err)
        }
    }
}

#[async_trait]
impl<F, Fut, R, E1, E2, E3> FunctionRouteHandler<(E1, E2, E3)> for F
where
    F : (Fn(E1, E2, E3) -> Fut) + Send + Sync + 'static + Clone,
    Fut : Future<Output = R> + Send + 'static,
    R : IntoResponse + Send + 'static,

    E1 : FromRequest + Send,
    E2 : FromRequest + Send,
    E3 : FromRequest + Send,
{
    async fn handle_request(&self, request: HttpRequest) -> HttpResponse {
        let extract_values = || -> Result<(E1, E2, E3), ExtractorError> {
            let e1value = E1::from_request(0, &request)?;
            let e2value = E2::from_request(1, &request)?;
            let e3value = E3::from_request(2, &request)?;
            Ok((e1value, e2value, e3value))
        };

        match extract_values() {
            Ok((e1value, e2value, e3value)) => self(e1value, e2value, e3value).await.into_response(),
            Err(err) => handle_function_failure(err)
        }
    }
}