use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }
    //todo!("Implement read db/auth logic");
    //todo!("Set cookie");

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}
