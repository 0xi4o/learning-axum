#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // hello world
    hc.do_get("/hello?name=Ilango").await?.print().await?;
    hc.do_get("/hello2/Ilango").await?.print().await?;

    // static
    // hc.do_get("/src/main.rs").await?.print().await?;

    // login
    let req_login = hc.do_post(
        "/api/login",
        json!({ "username": "demo1", "pwd": "welcome" }),
    );
    req_login.await?.print().await?;

    Ok(())
}
