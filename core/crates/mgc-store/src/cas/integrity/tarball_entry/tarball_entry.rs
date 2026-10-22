// Public data-only input item for tarball imports.
// DTO công khai chỉ chứa dữ liệu đầu vào import tarball.

#[derive(Debug, Clone)]
pub struct TarballEntry {
    pub path: String,
    pub data: Vec<u8>,
    pub executable: bool,
}
