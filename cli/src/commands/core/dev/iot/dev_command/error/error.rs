//! Typed errors for IoT dev decisions owned by this core.
//! Lỗi typed cho quyết định dev do IoT core sở hữu.

#[derive(Debug, thiserror::Error)]
pub enum IotDevError {
    #[error("'mgc dev' for '{framework}' iot framework is not implemented yet")]
    UnsupportedFramework { framework: String },
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
