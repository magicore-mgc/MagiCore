//! Typed package-scope failure mapped to the stable CLI diagnostic.
//! Lỗi scope package có kiểu, ánh xạ về diagnostic CLI ổn định.

#[derive(Debug, thiserror::Error)]
#[error("signature package scope is invalid")]
pub(in crate::commands::audit::signatures) struct PackageScopeError;

impl PackageScopeError {
    /// Preserve the established command-level message and exit contract.
    /// Giữ nguyên thông điệp ở command boundary và hợp đồng exit.
    pub fn into_cli(self) -> anyhow::Error {
        crate::error::trusted_signatures_invalid()
    }
}
