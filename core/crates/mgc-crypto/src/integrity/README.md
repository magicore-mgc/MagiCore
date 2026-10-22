# Integrity owner / Owner integrity

Owns SRI values and BLAKE3 verification/computation. The public paths remain `mgc_crypto::integrity::{SriHash, IntegrityVerifier}`. File verification preserves its full-file read behavior; file computation delegates to bounded-memory BLAKE3 streaming.

Sở hữu value SRI và verify/tính BLAKE3. Public path giữ nguyên `mgc_crypto::integrity::{SriHash, IntegrityVerifier}`. Verify file giữ cách đọc toàn file; compute file ủy thác cho BLAKE3 streaming bộ nhớ giới hạn.

- `sri_hash/` owns the public SRI value, parser and Display representation. / `sri_hash/` sở hữu value SRI public, parser và biểu diễn Display.
- `verify/` owns in-memory algorithm dispatch and mismatch policy. / `verify/` sở hữu dispatch thuật toán trong bộ nhớ và policy mismatch.
- `compute/` owns data-to-SRI computation. / `compute/` sở hữu tính SRI từ dữ liệu.
- `file/` owns file I/O entry points. / `file/` sở hữu entry point I/O file.
- `error/` maps local typed failures to the existing `CryptoError` surface. / `error/` map lỗi typed local về bề mặt `CryptoError` hiện có.
