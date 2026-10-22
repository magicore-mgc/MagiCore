use std::path::PathBuf;

/// Cache dir: MGC_DATA_DIR/templates (override qua MGC_TEMPLATES_DIR).
pub fn templates_cache_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("MGC_TEMPLATES_DIR") {
        return PathBuf::from(dir);
    }
    mgc_platform::paths::mgc_data_dir().join("templates")
}
