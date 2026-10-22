//! Parse optimizer `.env` files into a string map.
//! (Parse file `.env` của optimizer thành map chuỗi.)

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// Parse `.env` format using the project-safe text reader.
/// Parse định dạng `.env` bằng bộ đọc text an toàn của project.
pub(crate) fn parse_env_file(path: &Path) -> Result<HashMap<String, String>> {
    let content = crate::commands::optimizer::env_loader::project_files::read_optimizer_text(path)?;

    let mut vars = HashMap::new();

    for (line_index, line) in content.lines().enumerate() {
        let line = line.trim();

        // Skip empty lines and comments.
        // Bỏ qua dòng trống và comment.
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse KEY=value format.
        // Phân tích định dạng KEY=value.
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().to_string();
            let value = crate::commands::optimizer::env_loader::env_quote::strip_env_quotes(
                &value,
                line_index + 1,
            )?;
            vars.insert(key, value);
        }
    }

    Ok(vars)
}
