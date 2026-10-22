# quantize

English: Owns the `quantize` responsibility for the CLI model command.
Tiếng Việt: Sở hữu trách nhiệm `quantize` của lệnh model CLI.

## Boundary / Ranh giới

- Implementation lives in `quantize.rs`; `mod.rs` only declares and re-exports the owner.
- Preserve the existing `mgc model` command contract and its failure behavior.
- Keep this owner focused; shared behavior remains in the narrow owner that defines it.
## Error boundary / Ranh giới lỗi

Error boundary: `QuantizeError` reports that native quantization is unsupported; this owner never starts an external process.
Ranh giới lỗi: `QuantizeError` báo native quantization chưa được hỗ trợ; owner này không khởi chạy tiến trình ngoài.
