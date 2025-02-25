pub mod handler;
mod tree;
pub mod func;
pub mod flow;
#[cfg(feature = "macros")]
pub mod macros;

use crate::extractor::state::{Resource, ResourceMap};
use crate::router::flow::RequestFlow;
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use crate::router::func::FunctionRouteHandler;
use crate::router::handler::RouteHandler;
use crate::router::tree::RouteNode;
use async_trait::async_trait;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;
use crate::middleware::{Middleware, MiddlewareChain};

#[async_trait]
pub trait HttpRouter {
    fn insert_handler<Handler>(self, path: &str, handler: Handler) -> Self where
        Handler : RouteHandler + Send + Sync + 'static + Clone;

    fn add_handler<Ext, Handler>(self, path: &str, handler: Handler) -> Self where
        Ext: Send + Sync + 'static,
        Handler: FunctionRouteHandler<Ext> + Sized + Send + 'static;

    fn add_resource<T>(self, resource: T) -> Self where
        T: Clone + Send + Sync + 'static;

    fn add_endpoint<Ext, Handler>(self, endpoint: (&str, Handler)) -> Self
    where
        Ext: Send + Sync + 'static,
        Handler: FunctionRouteHandler<Ext> + Sized + Send + 'static;

    fn add_middleware<M>(self, middleware: M) -> Self where
        M: Middleware + Send + Sync + 'static;

    async fn route(&self, request: HttpRequest) -> Option<HttpResponse>;
}

pub struct Router {
    root: RouteNode,
    resources: ResourceMap,
    middlewares: MiddlewareChain
}

impl Router {
    pub fn new() -> Self {
        Router {
            root: RouteNode::new(),
            resources: HashMap::new(),
            middlewares: MiddlewareChain::new()
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
        if self.resources.contains_key(&TypeId::of::<Resource<T>>()) {
            panic!("Resource of type {} already exists in the router", std::any::type_name::<T>());
        }
        self.resources.insert(TypeId::of::<Resource<T>>(), Box::new(Resource::new(resource)));
        self
    }

    fn add_endpoint<Ext, Handler>(self, endpoint: (&str, Handler)) -> Self
    where
        Ext: Send + Sync + 'static,
        Handler: FunctionRouteHandler<Ext> + Sized + Send + 'static,
    {
        self.add_handler(endpoint.0, endpoint.1)
    }

    fn add_middleware<M>(mut self, middleware: M) -> Self
    where
        M: Middleware + Send + Sync + 'static,
    {
        self.middlewares.add_middleware(middleware);
        self
    }

    async fn route(&self, mut request: HttpRequest) -> Option<HttpResponse> {
        let route = self.root.find(&request.path)?;
        let handler = route.handler;
        if !handler.handles_method(request.method) {
            return None
        }
        let flow = Arc::new(RequestFlow::new(self.clone_resources()));
        request.set_flow(flow.clone());
        self.middlewares.apply_before(&mut request);

        request.set_route_path(route.into_path_values());
        let mut response = handler.handle(request).await;

        self.middlewares.apply_after(flow, &mut response);
        Some(response)
    }
}

impl Router {
    fn clone_resources(&self) -> ResourceMap {
        self.resources.iter()
            .map(|(type_id, resource)| (*type_id, resource.clone_box()))
            .collect()
    }
}