pub mod alpha;
pub mod beta;

use async_trait::async_trait;

#[async_trait]
pub trait HttpProtocol {
    async fn connect(&mut self, addr: &str) -> Result<(), HttpProtocolError>;

    async fn listen(&'static self) -> Result<(), HttpProtocolError>;
}

#[derive(Debug)]
pub struct HttpProtocolError {
    pub message: String
}