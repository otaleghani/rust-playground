use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{Error, Result};

use super::AUTH_TOKEN;

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }
    //todo!("Implement read db/auth logic");

    cookies.add(Cookie::new(crate::web::AUTH_TOKEN, "user-1.exp.sign"));

    //todo!("Set cookie");

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}
