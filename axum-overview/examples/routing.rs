use axum::extract::{Path, Query};
use axum::middleware;
use axum::routing::get;
use axum::{
    response::{Html, IntoResponse, Response},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let nested = Router::new().nest("/sus", router_hello());

    let routes_all = Router::new()
        .merge(nested)
        .merge(router_hello())
        .merge(route_read().layer(middleware::map_response(main_response_mapper)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Serving on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
}

// Routers must be not async
fn router_hello() -> Router {
    Router::new().route("/hello", get(handler_hello))
}

fn route_read() -> Router {
    Router::new()
        .route("/params", get(handler_extract_params))
        .route("/path/{name}", get(handler_extract_path))
}

// Handlers must be async
async fn handler_hello() -> impl IntoResponse {
    Html(format!("<h1>Hello</h1>"))
}

// Extractors
async fn handler_extract_path(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("<h1>Hello, {name}</h1>"))
}

#[derive(Debug, Deserialize)]
struct ParamsTest {
    filter: Option<String>,
    _pagination_page: Option<u64>,
    _pagination_quantity: Option<u64>,
}

async fn handler_extract_params(Query(params): Query<ParamsTest>) -> impl IntoResponse {
    let filter = params.filter.as_deref().unwrap_or("");
    Html(format!("<h1>Params.filter: {filter}</h1>"))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("got one");
    res
}
