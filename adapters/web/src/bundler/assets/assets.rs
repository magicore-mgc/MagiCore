//! Output HTML and public-directory materialization.
//! Materialize HTML output và thư mục public.

use super::super::types::BundlerConfig;
use std::path::Path;

/// Generates or rewrites the build index, then copies the entry's public assets.
/// Tạo hoặc rewrite index build, sau đó copy public asset cạnh entry.
pub async fn process_assets(config: &BundlerConfig) -> Result<(), anyhow::Error> {
    let root = super::super::workspace::detect_working_dir(&config.entry).unwrap_or_else(|| {
        config
            .entry
            .parent()
            .unwrap_or(Path::new("."))
            .to_path_buf()
    });
    materialize_index_html(&root, config)?;

    let public_dir = config
        .entry
        .parent()
        .unwrap_or(Path::new("."))
        .join("public");
    if public_dir.exists() {
        let destination = config.output_dir.join("public");
        copy_dir_all(&public_dir, &destination)?;
    }
    Ok(())
}

/// Rewrites an existing entry reference or emits the established fallback page.
/// Rewrite reference entry hiện có hoặc tạo trang fallback hiện hành.
fn materialize_index_html(root: &Path, config: &BundlerConfig) -> Result<(), anyhow::Error> {
    let output = config.output_dir.join("index.html");
    let bundle_name = config
        .entry
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| format!("{stem}.js"))
        .unwrap_or_else(|| "main.js".to_string());

    let source_html = root.join("index.html");
    if source_html.exists() {
        let mut html = std::fs::read_to_string(&source_html)?;
        if let Ok(relative_entry) = config.entry.strip_prefix(root) {
            let relative_entry = relative_entry.to_string_lossy().replace('\\', "/");
            for pattern in [
                format!("src=\"/{relative_entry}\""),
                format!("src=\"{relative_entry}\""),
                format!("src='/{relative_entry}'"),
                format!("src='{relative_entry}'"),
            ] {
                let replacement = pattern.replace(&relative_entry, &bundle_name);
                html = html.replace(&pattern, &replacement);
            }
        }
        std::fs::write(output, html)?;
        return Ok(());
    }

    let title = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("MagiCore App");
    let html = format!(
        "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"UTF-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    <title>{title}</title>\n  </head>\n  <body>\n    <div id=\"root\"></div>\n    <script type=\"module\" src=\"/{bundle_name}\"></script>\n  </body>\n</html>\n"
    );
    std::fs::write(output, html)?;
    Ok(())
}

/// Copies a public directory recursively while retaining its existing layout.
/// Copy đệ quy thư mục public và giữ layout hiện tại.
fn copy_dir_all(source: &Path, destination: &Path) -> Result<(), anyhow::Error> {
    std::fs::create_dir_all(destination)?;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &destination.join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), destination.join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
