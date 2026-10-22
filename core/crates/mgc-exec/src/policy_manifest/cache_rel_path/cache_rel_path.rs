/// Relative cache path for the manifest under a data root.
/// Đường dẫn cache tương đối cho manifest dưới data root.
pub fn cache_rel_path() -> std::path::PathBuf {
    std::path::PathBuf::from("policy/exceptions.json")
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
