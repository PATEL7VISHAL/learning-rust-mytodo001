#![allow(unused)]
use anyhow::Result;
use serde_json::{json, Value};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8080").unwrap();
    //NOTE: checking hello api
    // client.do_get("/").await?.print().await?;
    // client.do_get("/hello").await?.print().await?;
    // client.do_get("/hello?name=vishal").await?.print().await?;
    // client.do_get("/hello/patel").await?.print().await?;

    //NOTE: checking fallback servies
    // client.do_get("/index.html").await?.print().await?;

    client
        .do_post("/api/login", json!({"username": "demo", "pwd":"pass"}))
        .await?
        .print()
        .await?;

    // add task
    client
        .do_post(
            "/api/task",
            json!({
               "title":"Write an SA"
            }),
        )
        .await?
        .print()
        .await?;

    // add task
    client
        .do_post(
            "/api/task",
            json!({
               "title":"Read an SA"
            }),
        )
        .await?
        .print()
        .await?;

    // add task
    client.do_delete("/api/task?task_id=1").await?.print().await?;

    // list task
    client.do_get("/api/task").await?.print().await?;

    Ok(())
}
