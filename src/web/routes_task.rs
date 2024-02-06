use crate::{
    ctx::Ctx,
    model::{AddTask, ModelController, RemoveTask, Task},
    Result,
};
use axum::{
    extract::{State, Query},
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

use super::mw_auth::mw_ctx_resolver;

pub fn routes() -> Router {
    let mc = ModelController::new();
    Router::new().route(
        "/task",
        get(handler_get_tasks)
            .post(handler_add_task)
            .delete(handler_remove_task)
            .with_state(mc.clone())
            .layer(middleware::from_fn_with_state(mc, mw_ctx_resolver)),
    )
}

async fn handler_get_tasks(ctx: Ctx, State(mc): State<ModelController>) -> Result<Json<Vec<Task>>> {
    println!("->> {:<20} - get tasks", "HANDLER");
    let tasks = mc.get_tasks(ctx).await?;
    Ok(Json(tasks))
}

async fn handler_add_task(
    State(mut mc): State<ModelController>,
    ctx: Ctx,
    payload: Json<AddTask>,
) -> Result<Json<Value>> {
    println!("->> {:<20} - add tasks", "HANDLER");
    // async fn handle_add_task(payload: Json<AddTaskPayload>) -> impl IntoResponse {
    let task = mc.add_task(ctx, payload.0).await?;

    Ok(Json(json!({
    "result": {
        "success": true,
        "task": task
        }
    })))
}

async fn handler_remove_task(
    State(mut mc): State<ModelController>,
    ctx: Ctx,
    Query(remove_task): Query<RemoveTask>,
) -> Result<Json<Value>> {
    println!("->> {:<20} - remove tasks", "HANDLER");
    let task = mc.remove_task(remove_task).await?;

    Ok(Json(json!({
        "result":{
            "success": true,
            "task":task
        }
    })))
}
