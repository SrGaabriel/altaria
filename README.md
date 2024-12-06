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
        "/test/{id}" => handler
    };

    Server::builder()
        .local_port(8080)
        .router(router)
        .start()
        .await
        .unwrap()
}
```