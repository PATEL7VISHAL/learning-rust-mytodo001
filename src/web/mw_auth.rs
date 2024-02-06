use crate::{ctx::Ctx, model::ModelController, web::AUTH_TOKEN, Error, Result};
use axum::{
    async_trait,
    body::Body,
    extract::{FromRequestParts, Request, State},
    middleware::Next,
    response::Response,
};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<20} - mw_ctx_resolver", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNotAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, exp, sign)) => Ok(Ctx::new(user_id)),
        Err(e) => Err(e),
    };
    if result_ctx.is_err() && matches!(result_ctx, Err(Error::AuthFailNotAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }
    req.extensions_mut().insert(result_ctx);
    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;
    async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &S) -> Result<Self> {
        println!("->> {:<20} - Ctx", "EXTRACTOR");
        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormet)?;
    let user_id: u64 = user_id
        .parse()
        .map_err(|e| Error::AuthFailTokenWrongFormet)?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}
