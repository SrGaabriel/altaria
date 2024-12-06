# 🌌️ altaria

Altaria is an asynchronous, memory-safe, blazingly fast HTTP server written in Rust. It currently supports HTTP1.1 parsing and encoding and HTTP2 parsing.

```rust
#[tokio::main]
async fn main() {
    let handler = function_handler(|_| async {
        (HttpStatusCode::Unauthorized, "Hello, World!")
    });

    let router = router! {
        "/hello" => handler
        "/baba/{id}" => handler
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
```