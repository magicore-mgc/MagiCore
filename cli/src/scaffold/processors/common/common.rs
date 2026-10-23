//! Shared scaffold file helpers used by every core processor.
//! Helper ghi file dùng chung cho mọi processor scaffold.

use std::path::Path;

use anyhow::Result;

/// Ghi file — atomic-ish: tạo parent dirs trước khi write.
pub(crate) fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)?;
    Ok(())
}

pub(crate) fn slugify(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}
