use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

pub fn routes() -> Router {
    Router::new().route("/", get(handler_index))
}

async fn handler_index() -> impl IntoResponse {
    println!("->> {:<20} - handler_index", "HANDLER");
    return Html("<Strong><center>Root</center></Strong>");
}
