#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "password": "welcome"
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}
