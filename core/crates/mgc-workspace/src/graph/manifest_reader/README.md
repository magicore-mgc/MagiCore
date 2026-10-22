# Workspace manifest reader / Đọc manifest workspace

Coordinates the existing manifest priority (`package.json`, then `Cargo.toml`, then `pyproject.toml`) and exposes the stable `read_package_manifest` function. Language-specific parsing is delegated to sibling owners. Missing and malformed Cargo/Python manifests still fall through; an existing malformed `package.json` still returns an error. `Path::exists` symlink behavior is intentionally unchanged and requires a separate security fix.
