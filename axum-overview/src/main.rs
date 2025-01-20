#![allow(unused)]

use std::net::SocketAddr;

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
//use axum::response::Extension
use axum::routing::{get, get_service};
use axum::{Router, ServiceExt};
use serde::Deserialize;
use tower_http::services::ServeDir;

mod error;
mod web;
pub use self::error::{Error, Result};

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(routes_other())
        .merge(web::routes_login::routes())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on addr\n");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all.into_make_service()).await;
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/helloparams", get(handler_parameters))
        .route("/hello/{name}", get(handler_path))
}

fn routes_other() -> Router {
    Router::new().route("/other", get(handler_hello))
}

fn routes_static() -> Router {
    Router::new().nest_service("/files/", get_service(ServeDir::new("./")))
}

async fn handler_hello() -> impl IntoResponse {
    println!("{:<12} - handler_hello", "HANDLER");
    Html("<h1>Hello sus!</h1>")
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_parameters(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("{:<12} - handler_parameters", "HANDLER");
    let name = params.name.as_deref().unwrap_or("world!");
    Html(format!("<h1>Hello {name}!</h1>"))
}

async fn handler_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("{:<12} - handler_path", "HANDLER");

    Html(format!("<h1>Hello {name}"))
}
