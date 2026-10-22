# cas_pull

English: Owns the `cas_pull` responsibility for the CLI model command.
Tiếng Việt: Sở hữu trách nhiệm `cas_pull` của lệnh model CLI.

## Boundary / Ranh giới

- Implementation lives in `cas_pull.rs`; `mod.rs` only declares and re-exports the owner.
- Preserve the existing `mgc model` command contract and its failure behavior.
- Keep this owner focused; shared behavior remains in the narrow owner that defines it.
## Error boundary / Ranh giới lỗi

Error boundary: `CasPullError` rejects an unsupported source and a zero-byte limit before any download starts.
Ranh giới lỗi: `CasPullError` chặn nguồn không hỗ trợ và giới hạn 0 byte trước khi bắt đầu tải.
