#![allow(unused)]

use std::net::SocketAddr;

use axum::extract::{Path, Query};
use axum::handler::HandlerWithoutStateExt;
use axum::http::{Method, Uri};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Json, Router, ServiceExt};
use ctx::Ctx;
use log::log_request;
use model::ModelController;
use serde::Deserialize;
use serde_json::json;
use tower_cookies::cookie::time::format_description::modifier::UnixTimestampPrecision;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod ctx;
mod error;
mod log;
mod model;
mod web;
pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(routes_other())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on addr\n");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
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

async fn main_response_mapper(
    ctx: Result<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("main_response_mapper");
    let uuid = uuid::Uuid::new_v4();
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new response
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!(" => client_error_body: {client_error_body}");
            (*status_code, Json(client_error_body)).into_response()
        });

    let ctx_option = match ctx {
        Ok(val) => Some(val),
        Err(_) => None,
    };
    let client_error = client_status_error.unzip().1;
    log_request(
        uuid,
        req_method,
        uri,
        ctx_option,
        service_error,
        client_error,
    )
    .await;

    error_response.unwrap_or(res)
}
