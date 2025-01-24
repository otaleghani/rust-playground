use axum::{http::StatusCode, response::IntoResponse};
use std::sync::Arc;

use crate::{model, web};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // -- Modules
    Model(model::Error),
    CtxExt(web::mw_auth::CtxExtError),
}

// region:    --- Froms
impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Self::Model(val)
    }
}
// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
//

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}
