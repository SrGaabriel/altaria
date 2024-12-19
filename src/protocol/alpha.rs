use std::panic::AssertUnwindSafe;
use crate::encoder::alpha::AlphaHttpEncoder;
use crate::encoder::format::{DefaultHttpResponseFormatter, HttpResponseFormatter};
use crate::headers;
use crate::parser::alpha::AlphaHttpParser;
use crate::protocol::{HttpProtocol, HttpProtocolError};
use crate::response::{HttpResponse, HttpStatusCode};
use crate::router::{HttpRouter, Router};
use anyhow::bail;
use async_trait::async_trait;
use std::sync::Arc;
use futures::FutureExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub struct AlphaHttpProtocol {
    socket: Option<TcpListener>,
    router: Arc<Option<Router>>,
    parser: Arc<AlphaHttpParser>,
    encoder: Arc<AlphaHttpEncoder>,
    formatter: Arc<Box<dyn HttpResponseFormatter + Send + Sync>>
}

impl AlphaHttpProtocol {
    pub fn new() -> Self {
        AlphaHttpProtocol {
            socket: None,
            router: Arc::new(None),
            parser: Arc::new(AlphaHttpParser::new()),
            encoder: Arc::new(AlphaHttpEncoder::new()),
            formatter: Arc::new(Box::new(DefaultHttpResponseFormatter::new()))
        }
    }

    pub fn link_router(router: Router) -> Self {
        AlphaHttpProtocol {
            socket: None,
            router: Arc::new(Some(router)),
            parser: Arc::new(AlphaHttpParser::new()),
            encoder: Arc::new(AlphaHttpEncoder::new()),
            formatter: Arc::new(Box::new(DefaultHttpResponseFormatter::new()))
        }
    }
}

#[async_trait]
impl HttpProtocol for AlphaHttpProtocol {
    fn set_router(&mut self, router: Router) {
        self.router = Arc::new(Some(router))
    }

    async fn connect(&mut self, addr: &str) -> crate::Result<()> {
        let socket = TcpListener::bind(addr).await.map_err(|_| HttpProtocolError::PortAlreadyInUse)?;
        self.socket = Some(socket);
        Ok(())
    }

    async fn listen(&'static self) -> crate::Result<()> {
        let socket = match &self.socket {
            Some(socket) => socket,
            None => bail!(HttpProtocolError::UnboundSocket)
        };

        loop {
            let (stream, addr) = match socket.accept().await {
                Ok(connection) => connection,
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                    continue;
                }
            };
            let router = self.router.clone();
            let parser = self.parser.clone();
            let encoder = self.encoder.clone();
            let formatter = self.formatter.clone();

            tokio::spawn(async move {
                let (read_half, mut write_half) = stream.into_split();
                let parsed = match parser.parse(addr, read_half).await {
                    Ok(request) => request,
                    Err(e) => {
                        eprintln!("Failed to parse request: {:?}", e);
                        return;
                    }
                };

                let router = Option::as_ref(&*router).expect("Router not set");

                let routed_response = async { AssertUnwindSafe(router.route(parsed)).catch_unwind().await }.await;
                let response: HttpResponse = match routed_response {
                    Ok(response) => response.unwrap_or_else(|| HttpResponse {
                        status_code: HttpStatusCode::NotFound,
                        headers: headers! {
                                ContentType: "text/plain"
                            },
                        body: vec![]
                    }),
                    Err(e) => {
                        eprintln!("Failed to route request: {:?}", e);
                        HttpResponse {
                            status_code: HttpStatusCode::InternalServerError,
                            headers: headers! {
                                ContentType: "text/plain"
                            },
                            body: vec![]
                        }
                    }
                };

                let formatted = formatter.format(response);
                let encoded = match encoder.encode(formatted) {
                    Ok(encoded) => encoded,
                    Err(e) => {
                        eprintln!("Failed to encode response: {}", e);
                        return;
                    }
                };

                if let Err(e) = write_half.write_all(&encoded).await {
                    eprintln!("Failed to write response: {}", e);
                    return;
                }

                if let Err(e) = write_half.flush().await {
                    eprintln!("Failed to flush response: {}", e);
                    return;
                }
            });
        }
    }
}

unsafe impl Send for AlphaHttpProtocol {}
unsafe impl Sync for AlphaHttpProtocol {}