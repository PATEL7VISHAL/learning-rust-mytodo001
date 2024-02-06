#![allow(unused)]

use axum::{
    http::{Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Json, Router,
};

mod ctx;
mod error;
mod log;
mod model;
mod web;

use ctx::Ctx;
pub use error::{Error, Result};
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

use crate::log::log_request;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("LISTERNING ON ... {:?}", listener.local_addr());
    let routes = Router::new()
        .merge(web::routes_index::routes())
        .merge(web::routes_hello::routes())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_task::routes())
        .layer(middleware::map_response(main_response_wrapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

async fn main_response_wrapper(
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    res: Response,
) -> Response {
    println!("->> {:<20} - main_response_mapper", "MIDDLERWARE");
    let uuid = Uuid::new_v4();

    let server_error = res.extensions().get::<error::Error>();
    let client_status_error = server_error.map(|se| se.client_status_and_error());
    let err_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error":{
                    "type":client_error.as_ref(),
                    "req_uuid":uuid.to_string(),
                }
            });
            println!("  ->> client_error_body: {client_error_body:?}");
            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, server_error, client_error);
    println!("");
    res
}

fn routes_static() -> Router {
    println!("->> {:<20} - fallback_servies", "HANDLER");
    Router::new().nest_service("/", get_service(ServeDir::new("./public/")))
}
