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
        .add_resource("Altaria")
        .add_handler("/", handler)
        .add_endpoint(endpoint!(greet))
        .add_endpoint(endpoint!(meet));

    Server::builder()
        .local_port(8080)
        .router(router)
        .start()
        .await
        .unwrap()
}

#[post("/greet/{name}")]
async fn greet(
    name: String
) -> String {
    format!("Hello, {name}")
}

#[get("/meet/{name}")]
async fn meet(
    name: String,
    Resource(me): Resource<&str>,
) -> String {
    format!("I'm, {me}! Hello, {name}")
}
```
