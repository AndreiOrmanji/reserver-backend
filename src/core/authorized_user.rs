use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use actix_web::{dev, http::StatusCode, Error, FromRequest, HttpRequest, ResponseError};
use actix_web_httpauth::headers::authorization::{Bearer, Scheme};
use futures_util::future::{err, ok, Ready};
use serde_derive::Deserialize;

#[derive(Debug)]
pub struct BadAuthHeaderError;

impl Display for BadAuthHeaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{{\"{}\":\"{}\"}}", "message", "test")
    }
}

impl ResponseError for BadAuthHeaderError {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthorizedUser {
    pub inner: String,
}

impl FromRequest for AuthorizedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        match req.headers().get(actix_web::http::header::AUTHORIZATION) {
            None => err(Error::from(BadAuthHeaderError)),
            Some(h) => match Bearer::parse(h) {
                Err(_) => err(Error::from(BadAuthHeaderError)),
                Ok(s) => ok(AuthorizedUser {
                    inner: s.token().to_string(),
                }),
            },
        }
    }
}
