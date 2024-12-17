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
            let (mut stream, addr) = match socket.accept().await {
                Ok(connection) => connection,
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                    continue;
                }
            };
            println!("Accepted connection from {}", addr);

            let router = self.router.clone();
            let parser = self.parser.clone();
            let encoder = self.encoder.clone();
            let formatter = self.formatter.clone();

            tokio::spawn(async move {
                let parsed = match parser.parse(&mut stream).await {
                    Ok(request) => request,
                    Err(e) => {
                        eprintln!("Failed to parse request: {:?}", e);
                        return;
                    }
                };

                let routed_response = if let Some(router) = router.as_ref() {
                    router.route(parsed).await
                } else {
                    None
                };

                let response: HttpResponse = match routed_response {
                    Some(response) => response,
                    _ => HttpResponse {
                        status_code: HttpStatusCode::NotFound,
                        headers: headers! {
                            ContentType: "text/plain"
                        },
                        body: vec![]
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

                if let Err(e) = stream.write_all(&encoded).await {
                    eprintln!("Failed to write response: {}", e);
                    return;
                }

                if let Err(e) = stream.flush().await {
                    eprintln!("Failed to flush response: {}", e);
                    return;
                }
            });
        }
    }
}

unsafe impl Send for AlphaHttpProtocol {}
unsafe impl Sync for AlphaHttpProtocol {}