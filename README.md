# 🌌️ altaria

Altaria is an asynchronous, memory-safe, blazingly fast HTTP server written in Rust.

Roadmap:
- [x] HTTP1.1 protocol
- [x] HTTP1.1 parsing
- [x] HTTP1.1 encoding
- [x] Routing
- [x] Resources (states)
- [x] Json
- [ ] Middlewares
- [ ] Websockets
- [ ] HTTP2
- [ ] TLS

> [!IMPORTANT]  
> This project is made mostly for educational/learning purposes. It is not recommended to use it in production. Maybe in the future, it will be production-ready.

```rust
#[tokio::main]
async fn main() {
    let handler = function_handler(|_| async {
        (HttpStatusCode::OK, "Hello, World!")
    });

    let router = Router::new()
        .add_resource(Arc::new(Mutex::new(State { count: 0 })))
        .add_handler("/", handler)
        .add_endpoint(endpoint!(greet))
        .add_endpoint(endpoint!(count));

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

#[post("/count")]
async fn count(
    Resource(state): Resource<SharedState>,
    JsonBody(update): JsonBody<CountUpdate>
) -> JsonBody<CountUpdate> {
    let mut state = state.lock().await;
    state.count = update.new_count;
    JsonBody(CountUpdate {
        new_count: state.count
    })
}
```
