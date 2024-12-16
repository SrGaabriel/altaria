pub mod handler;
mod tree;
pub mod func;

use std::sync::Arc;
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use crate::router::handler::RouteHandler;
use crate::router::tree::RouteNode;
use async_trait::async_trait;
use crate::extractor::state::{to_resource_map, Resource, ResourceObligations};
use crate::middleware::RequestFlow;
use crate::router::func::FunctionRouteHandler;

#[async_trait]
pub trait HttpRouter {
    fn insert_handler<Handler>(self, path: &str, handler: Handler) -> Self where
        Handler : RouteHandler + Send + Sync + 'static + Clone;

    fn add_handler<Ext, Handler>(self, path: &str, handler: Handler) -> Self where
        Ext: Send + Sync + 'static,
        Handler: FunctionRouteHandler<Ext> + Sized + Send + 'static;

    fn add_resource<T>(self, resource: T) -> Self where
        T: Clone + Send + Sync + 'static;

    async fn route(&self, request: HttpRequest) -> Option<HttpResponse>;
}

pub struct Router {
    root: RouteNode,
    resources: Vec<Box<dyn ResourceObligations + Send + Sync>>
}

impl Router {
    pub fn new() -> Self {
        Router {
            root: RouteNode::new(),
            resources: Vec::new()
        }
    }
}

#[async_trait]
impl HttpRouter for Router {
    fn insert_handler<Handler>(mut self, path: &str, handler: Handler) -> Self where
        Handler : RouteHandler + Send + Sync + 'static
    {
        self.root.insert(path, Box::new(handler));
        self
    }

    fn add_handler<Ext, Handler>(self, path: &str, handler: Handler) -> Self
    where
        Ext: Send + Sync + 'static,
        Handler: FunctionRouteHandler<Ext> + Sized + Send + 'static,
    {
        self.insert_handler(path, handler.into_route_handler())
    }

    fn add_resource<T>(mut self, resource: T) -> Self
    where
        T: Clone + Send + Sync + 'static
    {
        self.resources.push(Box::new(Resource(resource)));
        self
    }

    async fn route(&self, mut request: HttpRequest) -> Option<HttpResponse> {
        let route = self.root.find(&request.path)?;
        let handler = route.handler;
        let resources = to_resource_map(&self.resources);
        request.flow = Some(Arc::new(RequestFlow::new(resources)));
        request.set_path_values(route.values);
        Some(handler.handle(request).await)
    }
}

#[macro_export]
macro_rules! router {
    ($($key:expr => $value:expr)*) => {
        {
            use crate::router::func::FunctionRouteHandler;
            use crate::router::HttpRouter;
            let mut router = crate::router::Router::new();

            $(
                router.add_handler($key, Box::new($value.into_route_handler()));
            )*
            router
        }
    }
}