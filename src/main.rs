mod parser;
mod request;
mod encoder;
mod response;
mod util;
mod protocol;

use crate::encoder::format::HttpResponseFormatter;
use crate::protocol::HttpProtocol;
use std::future::IntoFuture;
use std::io::{Read, Write};
use tokio::io::AsyncWriteExt;

pub struct HttpServer {
    pub protocol: Box<dyn HttpProtocol>
}

impl HttpServer {
    pub fn http1() -> HttpServer {
        HttpServer {
            protocol: Box::new(protocol::alpha::AlphaHttpProtocol::new())
        }
    }

    pub fn http2() -> HttpServer {
        HttpServer {
            protocol: Box::new(protocol::beta::BetaHttpProtocol::new())
        }
    }

    pub async fn bind(&mut self, addr: &str) -> Result<&mut HttpServer, protocol::HttpProtocolError> {
        self.protocol.connect(addr).await.map(|_| self)
    }

    pub async fn listen(self) -> Result<(), protocol::HttpProtocolError> {
        let static_ref = Box::leak(self.protocol);
        static_ref.listen().await
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        let client = reqwest::Client::builder()
            .http2_prior_knowledge()
            .build()
            .unwrap();
        let time = std::time::Instant::now();
        for i in 0..1000 {
            client.post("http://localhost:8080/ghfsdghis")
                .body("Hello, world!")
                .send()
                .await;
        }
        let elapsed = time.elapsed().as_millis();
        println!("All requests completed in {}ms", elapsed);
    });

    let mut server = HttpServer::http2();
    server
        .bind("localhost:8080")
        .await
        .expect("Failed to connect to server");

    server
        .listen()
        .await
        .expect("Failed to start server");
}