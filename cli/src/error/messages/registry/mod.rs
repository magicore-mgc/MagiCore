//! Registry authentication and metadata diagnostics.
//! Lỗi xác thực registry và metadata.

mod error;

pub use error::{
    no_home_dir, no_token_in_response, registry_auth_password_required,
    registry_auth_password_too_large, registry_auth_request_failed, registry_auth_requires_https,
    registry_auth_response_invalid, registry_auth_response_too_large, registry_auth_status_failed,
    registry_auth_url_invalid, registry_auth_username_required, registry_fetch_failed,
    registry_missing_field, registry_url_https,
};
