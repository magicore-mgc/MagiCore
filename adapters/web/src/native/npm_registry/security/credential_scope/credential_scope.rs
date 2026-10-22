//! security credential scope.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

use crate::native::npm_registry::ScopedRegistryClient;
use anyhow::Result;

/// Build a request without separating the selected token from its path scope.
/// Native callers phải truyền cả scoped registry; artifact public ngoài scope không có token.
pub fn scoped_registry_request(
    registry: &mgc_config::chain::ScopedRegistry,
    target: &str,
) -> Result<reqwest::RequestBuilder> {
    ScopedRegistryClient::new(registry)?.get(target)
}
