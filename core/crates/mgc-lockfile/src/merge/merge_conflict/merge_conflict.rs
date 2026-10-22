// Public merge conflict data and display contract.
// Dữ liệu conflict merge công khai và contract hiển thị.
#[derive(Debug)]
pub struct MergeConflict {
    pub name: String,
    pub base_version: Option<String>,
    pub ours_version: String,
    pub theirs_version: String,
}

impl std::fmt::Display for MergeConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "conflict: package '{}' changed on both sides (base {}, ours {}, theirs {})",
            self.name,
            self.base_version.as_deref().unwrap_or("<none>"),
            self.ours_version,
            self.theirs_version
        )
    }
}
