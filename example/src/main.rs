use altaria::extractor::Path;
use altaria::request::HttpRequest;
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
        .add_handler("/", handler)
        .add_function_handler("/users/{name}", greet);

    Server::builder()
        .local_port(8080)
        .router(router)
        .start()
        .await
        .unwrap()
}

async fn greet(
    Path(name): Path<String>,
    request: HttpRequest
) -> impl IntoResponse {
    format!("Hello, {}!", name)
}