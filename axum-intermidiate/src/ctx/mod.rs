// region:    --- Modules

mod error;

use axum::response::IntoResponse;

pub use self::error::{Error, Result};

// endregion: --- Modules

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i64,
}

// Constructor.
impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: 0 }
    }

    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }
}

// Property Accessors.
impl Ctx {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

impl IntoResponse for Ctx {
    fn into_response(self) -> axum::response::Response {
        "sus".into_response()
    }
}
