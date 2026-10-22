//! Unique canonical temp store roots. / CAS root tạm duy nhất, đã canonicalize.

use std::fs;
use std::path::PathBuf;

pub(crate) fn tmp_store_dir(name: &str) -> PathBuf {
    // Canonicalize: macOS temp dirs live under /var (a symlink); store roots
    // must be symlink-free, so tests resolve the real path first.
    // Canonicalize: temp dir macOS nằm dưới /var (symlink); store root phải
    // không chứa symlink nên test resolve đường dẫn thật trước.
    let dir = std::env::temp_dir()
        .canonicalize()
        .unwrap_or_else(|_| std::env::temp_dir().to_path_buf())
        .join("magicore-cas-adversarial")
        .join(format!(
            "{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&dir).unwrap();
    dir
}
