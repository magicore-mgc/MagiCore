#[derive(Debug, Clone)]
pub struct DatabaseEntry {
    pub id: String,
    pub version: String,
    pub integrity: Option<String>,
    pub installed_at: u64,
}
