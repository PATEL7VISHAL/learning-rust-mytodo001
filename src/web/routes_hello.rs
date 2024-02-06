use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(handler_hello_query))
        .route("/hello/:name", get(handler_hello_path))
}

#[derive(Debug, Deserialize)]
pub struct HelloParams {
    name: Option<String>,
}

async fn handler_hello_query(Query(param): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<20} - handler_hello", "HANDLER");
    if let Some(name) = param.name {
        return Html(format!("hello {}", name));
    }
    return Html("hello".into());
}

async fn handler_hello_path(Path(param): Path<HelloParams>) -> impl IntoResponse {
    println!("->> {:<20} - handler_hello", "HANDLER");
    if let Some(name) = param.name {
        return Html(format!("hello {}", name));
    }
    return Html("hello".into());
}
