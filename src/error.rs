use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
use strum_macros::AsRefStr;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, AsRefStr, Serialize)]
pub enum Error {
    LoginFail,

    // Db
    DbError,

    //Model
    TaskRemoveFailIdNotFound,
    TaskRemoveFailTaskNotFound,
    TaskUpdateFailIdNotFound,
    TaskUpdateFailTaskNotFound,

    //Auth
    AuthFailCtxNotInRequestExt,
    AuthFailTokenWrongFormet,
    AuthFailNotAuthTokenCookie,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<20} - {self:?}", "INTO_RES");
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            Self::AuthFailCtxNotInRequestExt
            | Self::AuthFailTokenWrongFormet
            | Self::AuthFailNotAuthTokenCookie => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // Model
            Self::TaskRemoveFailIdNotFound | Self::TaskUpdateFailIdNotFound => {
                (StatusCode::FORBIDDEN, ClientError::INVALID_PARM)
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVER_ERROR),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)] // basically strum_macros::AsRefStr convert the number
// values in to string to send the client
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARM,
    SERVER_ERROR,
}
