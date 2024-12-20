use std::sync::Arc;
use tracing::Level;
use crate::middleware::{Middleware, MiddlewarePriority};
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use crate::router::flow::RequestFlow;

pub struct LoggingMiddleware {
}

impl LoggingMiddleware {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn subscribing(level: LogLevel) -> Self {
        tracing_subscriber::fmt()
            .with_max_level(<LogLevel as Into<Level>>::into(level))
            .init();
        Self::new()
    }
}

pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR
}

impl Into<Level> for LogLevel {
    fn into(self) -> Level {
        match self {
            LogLevel::TRACE => Level::TRACE,
            LogLevel::DEBUG => Level::DEBUG,
            LogLevel::INFO => Level::INFO,
            LogLevel::WARN => Level::WARN,
            LogLevel::ERROR => Level::ERROR
        }
    }
}

impl Middleware for LoggingMiddleware {
    fn before_priority(&self) -> Option<MiddlewarePriority> {
        Some(MiddlewarePriority::Monitor)
    }

    fn after_priority(&self) -> Option<MiddlewarePriority> {
        Some(MiddlewarePriority::Monitor)
    }

    fn act_before(&self, req: &mut HttpRequest) {
        let method = &req.method;
        let path = &req.path;

        tracing::event!(Level::DEBUG, "Request: {} {}", method, path);
    }

    fn act_after(&self, _flow: Arc<RequestFlow>, response: &mut HttpResponse) {
        let status = &response.status_code;

        tracing::event!(Level::DEBUG, "Response: {}", status);
    }
}