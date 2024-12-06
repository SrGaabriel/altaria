pub mod alpha;
pub mod beta;

use async_trait::async_trait;
use crate::router::{HttpRouter, Router};

#[async_trait]
pub trait HttpProtocol {
    fn set_router(&mut self, router: Router);

    async fn connect(&mut self, addr: &str) -> Result<(), HttpProtocolError>;

    async fn listen(&'static self) -> Result<(), HttpProtocolError>;
}

#[derive(Debug)]
pub struct HttpProtocolError {
    pub message: String
}

impl HttpProtocolError {
    fn new(message: &str) -> Self {
        HttpProtocolError {
            message: message.to_string()
        }
    }
}