#[cfg(feature = "logging")]
pub mod log;

use std::sync::Arc;
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use crate::router::flow::RequestFlow;

pub trait Middleware {
    fn before_priority(&self) -> Option<MiddlewarePriority> {
        Some(MiddlewarePriority::Standard)
    }

    fn after_priority(&self) -> Option<MiddlewarePriority> {
        None
    }

    fn act_before(&self, req: &mut HttpRequest);

    #[allow(unused_variables)]
    fn act_after(&self, flow: Arc<RequestFlow>, response: &mut HttpResponse) {}
}

pub struct MiddlewareChain {
    chain: Vec<Box<dyn Middleware + Send + Sync>>
}

impl MiddlewareChain {
    pub fn new() -> Self {
        MiddlewareChain {
            chain: Vec::new()
        }
    }

    pub fn add_middleware<M>(&mut self, middleware: M)
    where M: Middleware + Send + Sync + 'static {
        self.chain.push(Box::new(middleware));
    }

    pub fn apply_before(&self, req: &mut HttpRequest) {
        let sorted_middlewares = self
            .sort_middlewares(|middleware| middleware.before_priority().is_some());

        for middleware in sorted_middlewares {
            middleware.act_before(req);
        }
    }

    pub fn apply_after(&self, flow: Arc<RequestFlow>, response: &mut HttpResponse) {
        let sorted_middlewares = self
            .sort_middlewares(|middleware| middleware.after_priority().is_some());

        for middleware in sorted_middlewares {
            middleware.act_after(flow.clone(), response);
        }
    }

    fn sort_middlewares<F>(&self, algorithm: F) -> Vec<&Box<dyn Middleware + Send + Sync>>
    where F: FnMut(&&Box<dyn Middleware + Send + Sync>) -> bool
    {
        let mut middleware_refs: Vec<&Box<dyn Middleware + Send + Sync>> = self.chain
            .iter()
            .filter(algorithm)
            .collect();
        middleware_refs.sort_by(|a, b| {
            match (a.before_priority(), b.before_priority()) {
                (Some(a), Some(b)) => a.cmp(&b),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal
            }
        });

        middleware_refs
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum MiddlewarePriority {
    Earliest,
    Early,
    Standard,
    Late,
    Latest,
    Monitor
}