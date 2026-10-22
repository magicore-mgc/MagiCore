# SHA-256 digest owner / Owner digest SHA-256

Owns SHA-256 over in-memory bytes and streamed filesystem paths. File hashing uses a fixed 64 KiB buffer and preserves the existing open/read error text.

Sở hữu SHA-256 trên bytes trong memory và file path đọc theo luồng. Hash file dùng buffer cố định 64 KiB và giữ nguyên thông điệp lỗi open/read.

Tests: `test/digest.rs` checks the standard `abc` digest and parity between byte and path hashing. No local error/type is required; errors remain the shared `MgResult` contract.

Test: `test/digest.rs` kiểm tra digest chuẩn của `abc` và parity giữa hash bytes/file. Không cần error/type cục bộ; lỗi giữ contract `MgResult` dùng chung.
