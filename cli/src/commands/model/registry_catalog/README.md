# registry_catalog

English: Owns the `registry_catalog` responsibility for the CLI model command.
Tiếng Việt: Sở hữu trách nhiệm `registry_catalog` của lệnh model CLI.

## Boundary / Ranh giới

- Implementation lives in `registry_catalog.rs`; `mod.rs` only declares and re-exports the owner.
- Preserve the existing `mgc model` command contract and its failure behavior.
- Keep this owner focused; shared behavior remains in the narrow owner that defines it.
## Error boundary / Ranh giới lỗi

Error boundary: `RegistryCatalogError` labels catalog listing failure; per-repository tag lookup keeps its existing best-effort behavior.
Ranh giới lỗi: `RegistryCatalogError` phân loại lỗi liệt kê catalog; lấy tag từng repository giữ nguyên cách best-effort.
