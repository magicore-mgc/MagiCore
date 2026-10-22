# registry_push

English: Owns the `registry_push` responsibility for the CLI model command.
Tiếng Việt: Sở hữu trách nhiệm `registry_push` của lệnh model CLI.

## Boundary / Ranh giới

- Implementation lives in `registry_push.rs`; `mod.rs` only declares and re-exports the owner.
- Preserve the existing `mgc model` command contract and its failure behavior.
- Keep this owner focused; shared behavior remains in the narrow owner that defines it.
## Error boundary / Ranh giới lỗi

Error boundary: `RegistryPushError` owns invalid input paths and push context; the underlying registry failure stays chained.
Ranh giới lỗi: `RegistryPushError` sở hữu lỗi đường dẫn đầu vào và ngữ cảnh push; lỗi registry gốc được giữ lại.
