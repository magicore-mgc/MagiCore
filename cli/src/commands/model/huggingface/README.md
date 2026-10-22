# huggingface

English: Owns the `huggingface` responsibility for the CLI model command.
Tiếng Việt: Sở hữu trách nhiệm `huggingface` của lệnh model CLI.

## Boundary / Ranh giới

- Implementation lives in `huggingface.rs`; `mod.rs` only declares and re-exports the owner.
- Preserve the existing `mgc model` command contract and its failure behavior.
- Keep this owner focused; shared behavior remains in the narrow owner that defines it.
## Error boundary / Ranh giới lỗi

Error boundary: `HuggingFaceError` owns pin, quota, request and integrity failures; the original request cause remains chained.
Ranh giới lỗi: `HuggingFaceError` sở hữu lỗi pin, quota, request và integrity; nguyên nhân request được giữ trong chuỗi lỗi.
