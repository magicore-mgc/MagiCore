//! types scoped client state.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

/// An instance-owned transport keeps one credential client and its immutable scope.
/// Không cache token toàn process; adapter giữ instance để tái dùng connection pool.
#[derive(Clone)]
pub struct ScopedRegistryClient {
    pub(in crate::native::npm_registry) scope: Option<reqwest::Url>,
    pub(in crate::native::npm_registry) token: Option<String>,
    pub(in crate::native::npm_registry) credential_client: Option<reqwest::Client>,
    pub(in crate::native::npm_registry) public_client: reqwest::Client,
}
