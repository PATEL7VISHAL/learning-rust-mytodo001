use std::time::{SystemTime, UNIX_EPOCH};
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::{ctx::Ctx, error::ClientError, Error};

pub fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    server_error: Option<&Error>,
    client_error: Option<ClientError>,
) {
    let timestemp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let error_type = server_error.map(|se| se.as_ref().to_string());
    let client_error_type = client_error.map(|ce| ce.as_ref().to_string());
    let error_data = serde_json::to_value(server_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestemp: timestemp.to_string(),
        user_id: ctx.map(|c| c.user_id()),
        req_path: uri.to_string(),
        req_method: req_method.to_string(),
        client_error_type,
        error_type,
        error_data,
    };
    println!("  ->> log_request: \n{}", json!(log_line))
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestemp: String,
    user_id: Option<u64>,
    req_path: String,
    req_method: String,
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
