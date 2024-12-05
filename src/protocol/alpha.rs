use crate::encoder::alpha::AlphaHttpEncoder;
use crate::encoder::format::{DefaultHttpResponseFormatter, HttpResponseFormatter};
use crate::encoder::HttpEncoder;
use crate::parser::alpha::AlphaHttpParser;
use crate::protocol::{HttpProtocol, HttpProtocolError};
use crate::response::{HttpResponse, HttpStatusCode};
use crate::headers;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub struct AlphaHttpProtocol {
    socket: Option<TcpListener>,
    parser: Arc<AlphaHttpParser>,
    encoder: Arc<AlphaHttpEncoder>,
    formatter: Arc<Box<dyn HttpResponseFormatter + Send + Sync>>
}

impl AlphaHttpProtocol {
    pub fn new() -> Self {
        AlphaHttpProtocol {
            socket: None,
            parser: Arc::new(AlphaHttpParser::new()),
            encoder: Arc::new(AlphaHttpEncoder::new()),
            formatter: Arc::new(Box::new(DefaultHttpResponseFormatter::new()))
        }
    }
}

#[async_trait]
impl HttpProtocol for AlphaHttpProtocol {
    async fn connect(&mut self, addr: &str) -> Result<(), HttpProtocolError> {
        let socket = TcpListener::bind(addr).await.map_err(|e| HttpProtocolError { message: e.to_string() })?;
        self.socket = Some(socket);
        Ok(())
    }

    async fn listen(&'static self) -> Result<(), HttpProtocolError> {
        let socket = match &self.socket {
            Some(socket) => socket,
            None => return Err(HttpProtocolError { message: "Socket not bound".to_string() })
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
                        eprintln!("Failed to encode response: {}", e.message);
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