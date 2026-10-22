//! Private state accumulated while reading a Terraform lock.
//! State cục bộ được tích lũy khi đọc Terraform lock.

#[derive(Debug, Default)]
pub(crate) struct TerraformLockState {
    pub(crate) providers: usize,
    pub(crate) current_provider: Option<String>,
    pub(crate) current_has_checksum: bool,
    pub(crate) failures: Vec<String>,
}

#[cfg(test)]
#[path = "test/types.rs"]
mod tests;
