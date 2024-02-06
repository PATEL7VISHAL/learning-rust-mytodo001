use crate::{web, Error, Result};
use axum::Json;
use axum::{response::IntoResponse, routing::post, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(handler_login))
}

async fn handler_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<20} - api_login", "HANDLER");

    if payload.username != "demo" || payload.pwd != "pass" {
        return Err(Error::LoginFail);
    }

    //NOTE: storing the cooking for client side
    //TODO: need to improve it
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result":{
            "success": true
        }
    }));
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
