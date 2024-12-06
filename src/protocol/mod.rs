pub mod alpha;
pub mod beta;

use async_trait::async_trait;
use thiserror::Error;
use crate::router::Router;

#[async_trait]
pub trait HttpProtocol {
    fn set_router(&mut self, router: Router);

    async fn connect(&mut self, addr: &str) -> crate::Result<()>;

    async fn listen(&'static self) -> crate::Result<()>;
}

#[derive(Debug, Error)]
pub enum HttpProtocolError {
    #[error("The address could not be bound. Port already in use?")]
    PortAlreadyInUse,
    #[error("You must bind the socket before listening")]
    UnboundSocket
}

