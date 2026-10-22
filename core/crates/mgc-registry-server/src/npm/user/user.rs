//! Register npm user and token endpoints.
//! Đăng ký endpoint user và token npm.

use super::{adduser, delete_user, revoke_token, whoami};
use crate::AppState;
use axum::{
    Router,
    routing::{delete, get, put},
};

pub(in crate::npm) fn routes() -> Router<AppState> {
    Router::new()
        .route("/-/user/{name}", put(adduser).delete(delete_user))
        .route("/-/user/token/{token}", delete(revoke_token))
        .route("/-/whoami", get(whoami))
}
