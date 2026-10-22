//! Public authentication facade / facade xác thực công khai.
//!
//! Re-exports the stable API while implementation remains in named owners.
//! Giữ đường dẫn API ổn định; logic nằm trong các owner có tên riêng.

mod auth;
mod debug;
mod header_value;
mod resolve_auth;

pub use auth::Auth;
pub use resolve_auth::resolve_auth;
