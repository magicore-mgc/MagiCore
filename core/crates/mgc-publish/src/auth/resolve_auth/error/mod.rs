//! Authentication resolution diagnostics / chẩn đoán phân giải xác thực.

mod error;
#[cfg(test)]
mod test;

pub(in crate::auth::resolve_auth) use error::{
    auth_not_found, auth_type_basic_missing, auth_type_token_missing,
};
