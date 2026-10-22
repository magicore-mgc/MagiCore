//! Registry login command owners and stable public entry points.
//! Các owner của lệnh đăng nhập registry và đường dẫn API ổn định.

mod credentials;
mod error;
#[path = "login/login.rs"]
mod login_implementation;
mod registry_user_creation;
mod token_storage;
mod types;

pub(crate) use credentials::read_password;
pub use login_implementation::run;
pub(crate) use registry_user_creation::create_registry_user;
pub use types::LoginArgs;
