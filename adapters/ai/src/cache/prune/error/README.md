# Prune error owner

This owner keeps cache I/O operation context, distinguishes invalid cache roots and refuses destructive handling of symbolic-link/unsupported entries. Conversion to `MgError::Anyhow` preserves the typed error and underlying `io::Error` source.

Owner này giữ context thao tác I/O, phân biệt cache root sai dạng và từ chối xử lý xóa entry symlink/không hỗ trợ. Chuyển qua `MgError::Anyhow` giữ typed error cùng source `io::Error` gốc.

Nearest source-chain coverage is in `test/error.rs`.

Test source-chain gần owner nằm tại `test/error.rs`.
