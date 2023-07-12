#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // hello world
    hc.do_get("/hello?name=Ilango").await?.print().await?;
    hc.do_get("/hello2/Ilango").await?.print().await?;

    // static
    // hc.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}
