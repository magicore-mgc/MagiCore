use anyhow::Result;
use std::path::{Path, PathBuf};

pub(crate) fn find_entry_point(root: &Path) -> Result<PathBuf> {
    let candidates = [
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

    for candidate in candidates {
        let path = root.join(candidate);
        if path.exists() {
            return Ok(path);
        }
    }

    let pkg_path = root.join("package.json");
    if pkg_path.exists() {
        let content = std::fs::read_to_string(&pkg_path)?;
        let pkg: serde_json::Value = serde_json::from_str(&content)?;

        if let Some(main) = pkg.get("main").and_then(|v| v.as_str()) {
            let path = root.join(main);
            if path.exists() {
                return Ok(path);
            }
        }
        if let Some(module) = pkg.get("module").and_then(|v| v.as_str()) {
            let path = root.join(module);
            if path.exists() {
                return Ok(path);
            }
        }
    }

    Err(anyhow::Error::new(super::error::EntryPointError::NotFound))
}
