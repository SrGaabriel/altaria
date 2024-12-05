use std::sync::Arc;
use async_trait::async_trait;
use tokio::net::TcpListener;
use crate::encoder::beta::BetaHttpEncoder;
use crate::encoder::format::{DefaultHttpResponseFormatter, HttpResponseFormatter};
use crate::parser::beta::BetaHttpParser;
use crate::protocol::{HttpProtocol, HttpProtocolError};

pub struct BetaHttpProtocol<'a> {
    socket: Option<TcpListener>,
    parser: BetaHttpParser<'a>,
    encoder: Arc<BetaHttpEncoder<'a>>,
    formatter: Arc<Box<dyn HttpResponseFormatter + Send + Sync>>
}

impl<'a> BetaHttpProtocol<'a> {
    pub fn new() -> Self {
        BetaHttpProtocol {
            socket: None,
            parser: BetaHttpParser::new(),
            encoder: Arc::new(BetaHttpEncoder::new()),
            formatter: Arc::new(Box::new(DefaultHttpResponseFormatter::new()))
        }
    }
}

#[async_trait]
impl<'a> HttpProtocol for BetaHttpProtocol<'a> {
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

            let mut parser = self.parser.clone();
            tokio::spawn(async move {
                match parser.parse(&mut stream).await {
                    Ok(request) => {
                    },
                    Err(e) => {
                        eprintln!("Failed to parse request from {}: {:?}", addr, e);
                    }
                }
            });
        }
    }
}