//! Discover the module entry point for a Web build.
//! Tìm entry point module cho build Web.

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Conventional browser entry candidates, highest precedence first.
/// Entry browser quy ước, ưu tiên từ cao xuống thấp.
///
/// These come from the framework templates MagiCore generates (Vite, Next.js,
/// Remix, …). They are checked before any package.json field so a project that
/// already declares a real framework entry keeps it.
/// Đây là entry từ template framework MagiCore sinh ra (Vite, Next.js, Remix, …).
/// Được kiểm trước mọi field package.json để project đã khai entry framework
/// thật vẫn giữ nguyên.
const BROWSER_ENTRIES: [&str; 20] = [
    "src/index.ts",
    "src/index.tsx",
    "src/main.ts",
    "src/main.tsx",
    "src/app.ts",
    "src/app.tsx",
    "index.ts",
    "index.tsx",
    "main.ts",
    "main.tsx",
    "src/index.js",
    "src/index.jsx",
    "src/main.js",
    "src/main.jsx",
    "src/app.js",
    "src/app.jsx",
    "index.js",
    "index.jsx",
    "main.js",
    "main.jsx",
];

/// Conventional Node server entry names, checked last.
/// Tên entry server Node quy ước, kiểm cuối cùng.
///
/// Framework generators that ship no `src/` (Express, and hand-written Node
/// servers) emit a top-level `app.js`/`server.js` and often only a `start`
/// script. These names let the bundled server build find its entry without a
/// package.json hint, while staying below explicit declarations.
/// Generator không có `src/` (Express, server Node viết tay) sinh `app.js`/
/// `server.js` ở gốc và thường chỉ có script `start`. Các tên này giúp build
/// server tìm được entry khi package.json không gợi ý, nhưng vẫn dưới khai báo
/// tường minh.
const SERVER_ENTRIES: [&str; 6] = [
    "server.js",
    "server.mjs",
    "server.cjs",
    "app.js",
    "index.js",
    "src/server.js",
];

/// Entry-point fields declared in `package.json`, highest precedence first.
/// Field entry khai trong `package.json`, ưu tiên từ cao xuống thấp.
///
/// `main` is the Node/bundler entry; `module` the ESM browser entry; `bin` the
/// command the package installs. A declared field is trusted over a
/// conventional filename because it is the project's own statement.
/// `main` là entry Node/bundler; `module` là entry ESM browser; `bin` là lệnh
/// package cài. Field được khai được tin hơn tên file quy ước vì đó là tuyên
/// bố của chính project.
const PACKAGE_ENTRY_FIELDS: [&str; 3] = ["main", "module", "bin"];

pub(crate) fn find_entry_point(root: &Path) -> Result<PathBuf> {
    for candidate in BROWSER_ENTRIES {
        let path = root.join(candidate);
        if path.exists() {
            return Ok(path);
        }
    }

    if let Some(path) = entry_from_package_fields(root) {
        return Ok(path);
    }

    for candidate in SERVER_ENTRIES {
        let path = root.join(candidate);
        if path.exists() {
            return Ok(path);
        }
    }

    Err(anyhow::Error::new(super::error::EntryPointError::NotFound))
}

/// Resolve an entry from the project's declared `main`/`module`/`bin` fields.
/// Resolve entry từ field `main`/`module`/`bin` đã khai của project.
///
/// Missing, non-string, or non-existent declared paths fall through to the next
/// field rather than failing: a declaration may point at a build output that
/// does not exist yet, and the conventional names are still valid fallbacks.
/// Field thiếu/không phải chuỗi/không tồn tại sẽ chuyển sang field kế tiếp thay
/// vì lỗi: khai báo có thể trỏ tới output chưa build, tên quy ước vẫn dùng được.
fn entry_from_package_fields(root: &Path) -> Option<PathBuf> {
    let pkg_path = root.join("package.json");
    if !pkg_path.exists() {
        return None;
    }
    let content = std::fs::read_to_string(&pkg_path).ok()?;
    let pkg: serde_json::Value = serde_json::from_str(&content).ok()?;

    for field in PACKAGE_ENTRY_FIELDS {
        let Some(value) = pkg.get(field) else {
            continue;
        };
        if let Some(declared) = declared_path(value) {
            let path = root.join(declared);
            if path.exists() {
                return Some(path);
            }
        }
    }
    None
}

/// Extract the first usable path string from a package field value.
/// Lấy chuỗi path dùng được đầu tiên từ giá trị field package.
///
/// `bin` may be either a path string or a map of command name to path; both
/// shapes are part of the npm contract, so accept the first map value.
/// `bin` có thể là chuỗi path hoặc map tên lệnh → path; cả hai dạng thuộc hợp
/// đồng npm, nên lấy value đầu tiên của map.
fn declared_path(value: &serde_json::Value) -> Option<&str> {
    match value {
        serde_json::Value::String(path) if !path.trim().is_empty() => Some(path.trim()),
        serde_json::Value::Object(map) => map
            .values()
            .find_map(|entry| entry.as_str().map(str::trim))
            .filter(|path| !path.is_empty()),
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/entry_point.rs"]
mod tests;
