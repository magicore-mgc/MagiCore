//! Explicit CLI token selection / chọn token CLI tường minh.

use crate::auth::Auth;

pub(in crate::auth::resolve_auth) fn resolve_cli_token(cli_token: Option<&str>) -> Option<Auth> {
    cli_token.map(|token| Auth {
        token: Some(token.to_string()),
        ..Default::default()
    })
}
