mod parser;
mod request;
mod encoder;
mod response;
mod util;

use std::future::IntoFuture;
use std::io::{Cursor, Read, Write};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use crate::encoder::format::HttpResponseFormatter;
use crate::parser::HttpParser;
use crate::response::HttpStatusCode;

pub struct HttpServer {
    pub parser: Arc<Box<dyn HttpParser + Send + Sync>>,
    pub encoder: Arc<Box<dyn encoder::HttpEncoder + Send + Sync>>,
    pub formatter: Arc<Box<dyn HttpResponseFormatter + Send + Sync>>
}

impl HttpServer {
    fn new() -> HttpServer {
        let parser = Box::new(parser::alpha::AlphaHttpParser {});
        let encoder = Box::new(encoder::beta::BetaHttpEncoder {});
        let formatter = Box::new(encoder::format::DefaultHttpResponseFormatter {});
        HttpServer { parser: Arc::new(parser), encoder: Arc::new(encoder), formatter: Arc::new(formatter) }
    }

    async fn listen(self, addr: &str) {
        let socket = TcpListener::bind(addr).await.expect("Failed to bind to address (port already in use?)");
        println!("Server listening on {}", addr);
        loop {
            let (mut stream, addr) = match socket.accept().await {
                Ok(connection) => connection,
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                    continue;
                }
            };
            println!("Connection established with {}", addr);

            let parser = self.parser.clone();
            let encoder = self.encoder.clone();
            let formatter = self.formatter.clone();

            tokio::spawn(async move {
                let now = std::time::Instant::now();
                println!("Processing request...");
                let parsed = match parser.parse(&mut stream).await {
                    Ok(request) => request,
                    Err(e) => {
                        eprintln!("Failed to parse request: {}", e.message);
                        return;
                    }
                };

                println!("Request processed in {}ms", now.elapsed().as_millis());

                let response = response::HttpResponse {
                    status_code: HttpStatusCode::OK,
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
                    eprintln!("Failed to flush stream: {}", e);
                    return;
                }

                println!("Response sent successfully: {}", String::from_utf8_lossy(&encoded));
            });
        }
    }
}

#[tokio::main]
async fn main() {
    // tokio::spawn(async {
    //     let client = reqwest::Client::new();
    //     for i in 0..10 {
    //         println!("Sending request {}", i);
    //         let future = client.post("http://localhost:8080")
    //             .body("Hello, world!")
    //             .send();
    //         println!("Request {} completed", i);
    //     }
    //     println!("All requests completed");
    // });

    let server = HttpServer::new();
    server
        .listen("localhost:8080")
        .await;
}