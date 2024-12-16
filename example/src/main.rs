use altaria::extractor::param::Param;
use altaria::extractor::state::Resource;
use altaria::response::HttpStatusCode;
use altaria::response::into::IntoResponse;
use altaria::router::{HttpRouter, Router};
use altaria::router::func::function_handler;
use altaria::Server;

#[tokio::main]
async fn main() {
    let handler = function_handler(|_| async {
        (HttpStatusCode::OK, "Hello, World!")
    });

    let router = Router::new()
        .add_resource("hello!")
        .add_handler("/", handler)
        .add_handler("/meet/{name}", meet)
        .add_handler("/users/{name}", greet);

    Server::builder()
        .local_port(8080)
        .router(router)
        .start()
        .await
        .unwrap()
}

async fn greet(
    Param(name): Param<String>,
) -> impl IntoResponse {
    format!("Hello, {name}")
}

async fn meet(
    Param(path): Param<String>,
    Resource(name): Resource<&str>,
) -> impl IntoResponse {
    format!("I'm, {name}! Hello, {path}")
}