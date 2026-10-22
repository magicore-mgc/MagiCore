# oci_pull

English: Owns the `oci_pull` responsibility for the CLI model command.
Tiếng Việt: Sở hữu trách nhiệm `oci_pull` của lệnh model CLI.

## Boundary / Ranh giới

- Implementation lives in `oci_pull.rs`; `mod.rs` only declares and re-exports the owner.
- Preserve the existing `mgc model` command contract and its failure behavior.
- Keep this owner focused; shared behavior remains in the narrow owner that defines it.
## Error boundary / Ranh giới lỗi

Error boundary: `OciPullError` classifies invalid locators and manifest requests; OCI client causes remain chained.
Ranh giới lỗi: `OciPullError` phân loại locator sai và lỗi lấy manifest; nguyên nhân từ OCI client được giữ lại.
