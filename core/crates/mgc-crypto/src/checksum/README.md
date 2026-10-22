# Checksum owner / Owner checksum

This owner exposes Adler-32 calculation and verification at the historical `mgc_crypto::checksum` path. Adler-32 is a quick, non-cryptographic checksum; do not use it for authentication, signatures or cryptographic integrity.

Owner này xuất phép tính và xác minh Adler-32 tại path lịch sử `mgc_crypto::checksum`. Adler-32 là checksum nhanh, không có tính mật mã; không dùng để xác thực, ký số hoặc bảo đảm integrity mật mã.

- `checksum.rs` owns the modulo-based Adler-32 calculation. / `checksum.rs` sở hữu phép tính Adler-32 theo modulo.
- `verification/` owns comparison with an expected checksum and preserves the historical `anyhow::Result<bool>` signature. / `verification/` sở hữu việc so sánh checksum kỳ vọng và giữ chữ ký `anyhow::Result<bool>` lịch sử.
- Focused tests live under each owner; public integration tests remain under the crate's `tests/`. / Test tập trung đặt dưới từng owner; integration test public vẫn ở `tests/` của crate.
- No local error type is needed: the current operations define no failing branch or recovery policy. / Không cần error type local vì các thao tác hiện tại không có nhánh thất bại hay policy phục hồi.
