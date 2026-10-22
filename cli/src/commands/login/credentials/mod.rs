//! Wire credential input and its local error type.
//! Nối nhập credential và kiểu lỗi cục bộ.

mod credentials;
mod error;

pub(crate) use credentials::read_password;
pub(super) use credentials::{prompt_text, read_password_inner};
pub(super) use error::{CredentialInputError, into_diagnostic as credential_input_diagnostic};
