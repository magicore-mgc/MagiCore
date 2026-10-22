# local_models

English: Owns the `local_models` responsibility for the CLI model command.
Tiếng Việt: Sở hữu trách nhiệm `local_models` của lệnh model CLI.

## Boundary / Ranh giới

- Implementation lives in `local_models.rs`; `mod.rs` only declares and re-exports the owner.
- Preserve the existing `mgc model` command contract and its failure behavior.
- Keep this owner focused; shared behavior remains in the narrow owner that defines it.
## Error boundary / Ranh giới lỗi

Error boundary: `LocalModelError` reports missing local models; path policy stays with `manifest_path`, and JSON diagnostics stay shared.
Ranh giới lỗi: `LocalModelError` báo model local không tồn tại; policy đường dẫn thuộc `manifest_path`, lỗi JSON dùng diagnostic chung.
