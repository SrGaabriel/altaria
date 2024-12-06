mod parser;
mod request;
mod encoder;
mod response;
mod util;
mod protocol;
mod router;

use crate::encoder::format::HttpResponseFormatter;
use crate::protocol::HttpProtocol;
use std::future::IntoFuture;
use std::io::{Read, Write};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use crate::response::{HttpResponse, HttpStatusCode};
use crate::response::into::IntoResponse;
use crate::router::func::function_handler;
use crate::router::Router;

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
    let handler = function_handler(|request| async {
        (HttpStatusCode::Unauthorized, "Hello, World!")
    });

    let router = router! {
        "/hello" => handler
    };

    let mut server = HttpServer::http1(router);
    server
        .bind("localhost:8080")
        .await
        .expect("Failed to connect to server");

    server
        .listen()
        .await
        .expect("Failed to start server");
}

#[tokio::test]
async fn send_requests() {
    let client = reqwest::Client::builder()
        .build()
        .unwrap();
    let time = std::time::Instant::now();
    for i in 0..1 {
        let request_future = client.post("http://localhost:8080/hello")
            .body("Hello, World!")
            .send();

        let response = tokio::time::timeout(Duration::from_secs(1), request_future)
            .await
            .expect("Request timed out")
            .expect("Unsuccesful request");
        println!("({}) Successful request: {}", response.status(), response.text().await.unwrap())
    }
    let elapsed = time.elapsed().as_millis();
    println!("All requests completed in {}ms", elapsed);
}