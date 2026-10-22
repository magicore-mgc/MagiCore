//! File metadata used by the PyPI simple API.
//! Metadata file dùng bởi PyPI simple API.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PypiFile {
    pub name: String,
    pub version: String,
    pub filename: String,
    pub digest: String, // sha256 digest in hex form — mã băm sha256 dạng hex
    pub size: i64,
    pub requires_python: Option<String>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
