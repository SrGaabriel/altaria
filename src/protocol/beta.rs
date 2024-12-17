use std::sync::Arc;
use anyhow::bail;
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use crate::encoder::beta::BetaHttpEncoder;
use crate::encoder::format::{DefaultHttpResponseFormatter, HttpResponseFormatter};
use crate::headers;
use crate::parser::beta::BetaHttpParser;
use crate::protocol::{HttpProtocol, HttpProtocolError};
use crate::response::{HttpResponse, HttpStatusCode};
use crate::router::Router;

pub struct BetaHttpProtocol<'a> {
    socket: Option<TcpListener>,
    parser: BetaHttpParser<'a>,
    encoder: BetaHttpEncoder<'a>,
    formatter: Arc<Box<dyn HttpResponseFormatter + Send + Sync>>
}

impl<'a> BetaHttpProtocol<'a> {
    pub fn new() -> Self {
        BetaHttpProtocol {
            socket: None,
            parser: BetaHttpParser::new(),
            encoder: BetaHttpEncoder::new(),
            formatter: Arc::new(Box::new(DefaultHttpResponseFormatter::new()))
        }
    }
}

#[async_trait]
impl<'a> HttpProtocol for BetaHttpProtocol<'a> {
    fn set_router(&mut self, router: Router) {
        todo!()
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

            let mut parser = self.parser.clone();
            let formatter = self.formatter.clone();
            let mut encoder = self.encoder.clone();
            tokio::spawn(async move {
                match parser.parse(&mut stream).await {
                    Ok(request) => {
                        let response = HttpResponse {
                            status_code: HttpStatusCode::ImATeapot,
                            headers: headers! {
                    ContentType: "text/plain"
                },
                            body: "Hello, world!".to_string().into_bytes()
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
                    },
                    Err(e) => {
                        eprintln!("Failed to parse request from {}: {:?}", addr, e);
                    }
                }
            });
        }
    }
}