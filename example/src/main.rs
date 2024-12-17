use altaria::extractor::state::Resource;
use altaria::response::HttpStatusCode;
use altaria::response::into::IntoResponse;
use altaria::router::{HttpRouter, Router};
use altaria::router::func::function_handler;
use altaria::{endpoint, Server};
use altaria_macros::{get, post};

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
) -> impl IntoResponse {
    format!("I'm, {me}! Hello, {name}")
}