mod parser;
mod request;

use std::io::Read;
use std::net::TcpListener;
use std::sync::Arc;
use crate::parser::HttpParser;

pub struct HttpServer {
    pub socket: TcpListener,
    pub parser: Arc<Box<dyn HttpParser + Send + Sync>>
}

impl HttpServer {
    fn new(addr: &str) -> HttpServer {
        let listener = TcpListener::bind(addr).unwrap();
        let parser = Box::new(parser::alpha::AlphaHttpParser {});
        HttpServer { socket: listener, parser: Arc::new(parser) }
    }

    fn listen(self) {
        println!("Server listening on {}", self.socket.local_addr().unwrap());
        loop {
            let (mut stream, addr) = self.socket.accept().unwrap();
            println!("Connection established with {}", addr);
            let parser = self.parser.clone();

            std::thread::spawn(move || {
                let now = std::time::Instant::now();
                let parsed = parser.parse(&mut stream);
                println!("Request processed in {}ms", now.elapsed().as_millis());
                if parsed.is_err() {
                    println!("Failed to parse request: {}", parsed.err().unwrap().message);
                    return;
                }

                println!("Parsed request: {:?}", parsed.unwrap());
            });
        }
    }
}

pub fn main() {
    let server = HttpServer::new("localhost:8080");
    server.listen();
}