# manifest_path

English: Owns the `manifest_path` responsibility for the CLI model command.
Tiếng Việt: Sở hữu trách nhiệm `manifest_path` của lệnh model CLI.

## Boundary / Ranh giới

- Implementation lives in `manifest_path.rs`; `mod.rs` only declares and re-exports the owner.
- Preserve the existing `mgc model` command contract and its failure behavior.
- Keep this owner focused; shared behavior remains in the narrow owner that defines it.
## Error boundary / Ranh giới lỗi

Error boundary: `ManifestPathError` classifies unsafe names and manifest paths; filesystem failures retain their I/O source.
Ranh giới lỗi: `ManifestPathError` phân loại tên và đường dẫn manifest không an toàn; lỗi filesystem giữ nguyên nguồn I/O.
