//! Parse classic Yarn lockfile text — parse text lockfile Yarn classic.

use anyhow::{Result, bail};

use crate::schema::Package;

use super::super::split_name_version;

// ---------------------------------------------------------------------------
// yarn — yarn.lock text (classic v1 syntax; berry bị reject fail-closed)
// yarn — parser giữ format v1 và từ chối Berry theo chế độ fail-closed.
// ---------------------------------------------------------------------------

pub(in crate::import) fn parse_yarn(content: &str) -> Result<Vec<Package>> {
    if content.contains("__metadata:") {
        bail!(
            "yarn berry lockfiles are not supported (detected '__metadata:') — refusing to guess"
        );
    }

    let mut out: Vec<Package> = Vec::new();
    let mut current_name: Option<String> = None;
    let mut version: Option<String> = None;
    let mut resolved = String::new();
    let mut integrity = String::new();

    let flush = |name: &mut Option<String>,
                 version: &mut Option<String>,
                 resolved: &mut String,
                 integrity: &mut String,
                 out: &mut Vec<Package>| {
        if let (Some(name), Some(version)) = (name.take(), version.take()) {
            out.push(Package {
                name,
                version,
                resolved: std::mem::take(resolved),
                integrity: std::mem::take(integrity),
                dependencies: vec![],
                ..Default::default()
            });
        } else {
            *name = None;
            *version = None;
        }
    };

    for line in content.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        if !line.starts_with(' ') {
            // A new header is `name@range:` and may quote or comma-separate multiple specs.
            // Header mới là `name@range:`; có thể có dấu nháy hoặc nhiều spec phân tách bằng dấu phẩy.
            flush(
                &mut current_name,
                &mut version,
                &mut resolved,
                &mut integrity,
                &mut out,
            );
            let header = line.trim_end_matches(':').trim().trim_matches('"');
            let first_spec = header.split(',').next().unwrap_or_default().trim();
            current_name = split_name_version(first_spec).map(|(n, _)| n);
        } else if let Some((field, value)) = line.trim().split_once(' ') {
            let value = value.trim().trim_matches('"');
            // Cú pháp yarn.lock: `version "x"` — field KHÔNG có dấu ':'
            // (yarn.lock syntax: bare field name, no trailing colon)
            match field.trim_end_matches(':') {
                "version" => version = Some(value.to_string()),
                "resolved" => resolved = value.to_string(),
                "integrity" => integrity = value.to_string(),
                _ => {}
            }
        }
    }
    flush(
        &mut current_name,
        &mut version,
        &mut resolved,
        &mut integrity,
        &mut out,
    );

    Ok(out)
}
