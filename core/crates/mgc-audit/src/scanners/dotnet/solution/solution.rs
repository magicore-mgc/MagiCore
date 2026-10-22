//! Discover root solutions and parse their `.csproj` declarations.
//! Tìm solution ở root và parse khai báo `.csproj`.

use std::path::{Path, PathBuf};

/// Return every root `.sln`; the collector sorts paths before reading.
/// Lấy mọi `.sln` ở root; collector sắp path trước khi đọc.
pub(in crate::scanners::dotnet) fn find_solution_files(project_root: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(project_root)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("sln"))
                .collect()
        })
        .unwrap_or_default()
}

/// Parse `.csproj` declarations and normalize Windows separators.
/// Parse khai báo `.csproj` và chuẩn hóa dấu phân cách Windows.
pub fn read_solution_projects(raw: &str) -> Vec<String> {
    let mut projects = Vec::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("Project(") {
            continue;
        }
        let mut parts = trimmed.splitn(2, '=');
        let _guid = parts.next();
        let Some(rest) = parts.next() else { continue };
        let mut quotes = Vec::new();
        let mut current = String::new();
        let mut in_quote = false;
        for character in rest.chars() {
            if character == '"' {
                if in_quote {
                    quotes.push(std::mem::take(&mut current));
                }
                in_quote = !in_quote;
            } else if in_quote {
                current.push(character);
            }
        }
        if quotes.len() >= 2 {
            let path = quotes[1].replace('\\', "/");
            if path.ends_with(".csproj") {
                projects.push(path);
            }
        }
    }
    projects
}
