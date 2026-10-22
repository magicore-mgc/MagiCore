//! Validate pub.dev package names before constructing registry paths.
//! Kiểm tra tên package trước khi ghép đường dẫn registry.

use super::super::error::CatalogError;
use mgc_types::MgResult;

pub(in crate::protocols::pubdev) fn validate_pub_package_name(name: &str) -> MgResult<()> {
    let mut bytes = name.bytes();
    if !bytes.next().is_some_and(|byte| byte.is_ascii_lowercase())
        || !bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return Err(CatalogError::InvalidPackageName(name.to_string()).into_mg_error());
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
