//! Cached JavaScript and optional stylesheet output.
//! JavaScript đã cache và stylesheet tùy chọn.

/// A bundled package response retained in the development-server session.
/// Response package đã bundle được giữ trong session dev server.
#[derive(Clone, Debug)]
pub struct CachedDep {
    pub js: String,
    pub css: Option<String>,
}

#[cfg(test)]
#[path = "test/types.rs"]
mod tests;
