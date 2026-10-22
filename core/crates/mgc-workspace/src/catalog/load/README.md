# Workspace catalog loading / Nạp catalog workspace

## English

Owns the current file-existence check, TOML read and catalog-section deserialization for magicore.workspace.toml. Missing files return None; parse and read errors retain the existing anyhow result. A symlink is currently followed and is not rejected here.

## Tiếng Việt

Quản lý kiểm tra file, đọc TOML và deserialize phần catalog của magicore.workspace.toml theo hành vi hiện tại. File không tồn tại trả None; lỗi đọc/parse giữ nguyên anyhow result. Symlink hiện được theo và chưa bị từ chối tại đây.
