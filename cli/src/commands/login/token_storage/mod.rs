//! Wire token path selection and persistence.
//! Nối chọn đường dẫn và lưu token.

mod error;
mod token_storage;

pub(super) use error::{TokenStorageError, into_diagnostic as token_storage_diagnostic};
pub(super) use token_storage::store_registry_token;
