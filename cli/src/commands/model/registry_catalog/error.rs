// Registry catalog failures — lỗi đọc catalog registry.
// Own the registry catalog failure classification — sở hữu phân loại lỗi catalog registry.

use std::fmt;

#[derive(Debug)]
pub(in crate::commands::model) enum RegistryCatalogError {
    ListingFailed,
}

impl fmt::Display for RegistryCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ListingFailed => f.write_str("catalog listing failed"),
        }
    }
}

impl std::error::Error for RegistryCatalogError {}
