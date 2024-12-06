pub mod handler;
mod tree;
pub mod func;

use async_trait::async_trait;
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use crate::router::handler::RouteHandler;
use crate::router::tree::{RouteHandlerPath, RouteNode};

#[async_trait]
pub trait HttpRouter {
    fn add_handler(&mut self, path: &str, handler: Box<dyn RouteHandler + Send + Sync + 'static>);
    
    async fn route(&self, request: HttpRequest) -> Option<HttpResponse>;
}

pub struct Router {
    root: RouteNode
}

impl Router {
    pub fn new() -> Self {
        Router {
            root: RouteNode::new()
        }
    }
}

#[async_trait]
impl HttpRouter for Router {
    fn add_handler(&mut self, path: &str, handler: Box<dyn RouteHandler + Send + Sync + 'static>) {
        self.root.insert(path, handler);
    }

    async fn route(&self, request: HttpRequest) -> Option<HttpResponse> {
        let route = self.root.find(&request.path)?;
        let handler = route.handler;
        Some(handler.handle(request).await)
    }
}

#[macro_export]
macro_rules! router {
    ($($key:expr => $value:expr),*) => {
        {
            use crate::router::HttpRouter;
            let mut router = crate::router::Router::new();
            $(
                router.add_handler($key, $value);
            )*
            router
        }
    }
}