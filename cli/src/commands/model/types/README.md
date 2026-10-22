# types

English: Owns the `types` responsibility for the CLI model command.
Tiếng Việt: Sở hữu trách nhiệm `types` của lệnh model CLI.

## Boundary / Ranh giới

- Implementation lives in `types.rs`; `mod.rs` only declares and re-exports the owner.
- Preserve the existing `mgc model` command contract and its failure behavior.
- Keep this owner focused; shared behavior remains in the narrow owner that defines it.
## Error boundary / Ranh giới lỗi

Error boundary: `ModelOptionsError` rejects source flags used with a non-HF model source.
Ranh giới lỗi: `ModelOptionsError` từ chối dùng flag của HF với nguồn model khác.
