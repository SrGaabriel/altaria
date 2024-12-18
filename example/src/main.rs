use std::sync::Arc;
use tokio::sync::Mutex;
use altaria::extractor::state::Resource;
use altaria::response::HttpStatusCode;
use altaria::response::into::IntoResponse;
use altaria::router::{HttpRouter, Router};
use altaria::router::func::function_handler;
use altaria::{endpoint, Server};
use altaria::json::JsonBody;
use altaria_macros::{get, post};

pub struct State {
    pub count: u32
}

type SharedState = Arc<Mutex<State>>;

#[tokio::main]
async fn main() {
    let handler = function_handler(|_| async {
        (HttpStatusCode::OK, "Hello, World!")
    });

    let router = Router::new()
        .add_resource(Arc::new(Mutex::new(State { count: 0 })))
        .add_handler("/", handler)
        .add_endpoint(endpoint!(greet))
        .add_endpoint(endpoint!(meet))
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

#[get("/meet/{name}")]
async fn meet(
    name: String,
    Resource(me): Resource<&str>,
) -> String {
    format!("I'm, {me}! Hello, {name}")
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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CountUpdate {
    pub new_count: u32
}