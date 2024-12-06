# 🌌️ altaria

Altaria is an asynchronous, memory-safe, blazingly fast HTTP server written in Rust. It currently supports HTTP1.1 parsing and encoding and HTTP2 parsing.

> [!IMPORTANT]  
> This project is made mostly for educational/learning purposes. It is not recommended to use it in production. Maybe in the future, it will be production-ready.

```rust
#[tokio::main]
async fn main() {
    let handler = function_handler(|_| async {
        (HttpStatusCode::OK, "Hello, World!")
    });

    let router = Router::new()
        .add_handler("/", handler)
        .add_function_handler("/users/{name}", greet);

    Server::builder()
        .local_port(8080)
        .router(router)
        .start()
        .await
        .unwrap()
}

async fn greet(Path(name): Path<String>, request: HttpRequest) -> impl IntoResponse {
    format!("Hello, {}!", name)
}
```
