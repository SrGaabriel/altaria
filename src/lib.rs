pub extern crate async_trait;
pub extern crate paste;

mod parser;
pub mod request;
mod encoder;
pub mod response;
mod protocol;
pub mod router;
pub mod extractor;
mod middleware;
#[cfg(feature = "json")]
pub mod json;
mod util;

use crate::protocol::HttpProtocol;
use crate::router::{Router};
use std::net::{Ipv4Addr, SocketAddr};
use thiserror::Error;

type Result<T> = anyhow::Result<T>;

pub struct HttpServer {
    pub protocol: Box<dyn HttpProtocol>
}

impl HttpServer {
    pub fn http1(router: Router) -> HttpServer {
        HttpServer {
            protocol: Box::new(protocol::alpha::AlphaHttpProtocol::link_router(router))
        }
    }

    pub fn http2() -> HttpServer {
        HttpServer {
            protocol: Box::new(protocol::beta::BetaHttpProtocol::new())
        }
    }

    pub fn set_router(&mut self, router: Router) {
        self.protocol.set_router(router)
    }

    pub async fn bind(&mut self, addr: &str) -> Result<&mut Self> {
        self.protocol.connect(addr).await.map(|_| self)
    }

    pub async fn listen(self) -> Result<()> {
        let static_ref = Box::leak(self.protocol);
        static_ref.listen().await
    }
}

impl Default for HttpServer {
    fn default() -> Self {
        HttpServer {
            protocol: Box::new(protocol::alpha::AlphaHttpProtocol::new())
        }
    }
}

pub struct Server {
    address: Option<SocketAddr>,
    router: Option<Router>
}

#[derive(Debug, Error)]
pub enum ServerBuildError {
    #[error("Address must be defined")]
    UndefinedAddress
}

impl Server {
    pub fn builder() -> Self {
        Server {
            address: None,
            router: None
        }
    }

    pub fn local_port(self, port: u16) -> Self {
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let addr = SocketAddr::new(ip.into(), port);
        self.address(addr)
    }

    pub fn addr(mut self, addr: &str) -> Self {
        self.address = Some(addr.parse().expect("Invalid address"));
        self
    }

    pub fn address(mut self, address: SocketAddr) -> Self {
        self.address = Some(address);
        self
    }

    pub fn router(mut self, router: Router) -> Self {
        self.router = Some(router);
        self
    }

    pub async fn start(self) -> Result<()> {
        let mut server = HttpServer::default();
        if let Some(router) = self.router {
            server.set_router(router)
        }
        let addr = self.address.ok_or(ServerBuildError::UndefinedAddress)?;

        server
            .bind(&addr.to_string())
            .await?;

        server
            .listen()
            .await?;
        Ok(())
    }
}