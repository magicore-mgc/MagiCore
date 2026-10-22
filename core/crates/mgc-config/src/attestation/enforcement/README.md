# Attestation enforcement / Enforcement chứng thực

This owner applies `off`, `warn` and `require` policies to validated or missing core markers. Separate owners preserve live-marker checks, missing-marker checks and one process-wide warning gate.

Owner này áp dụng policy `off`, `warn`, `require` cho marker core đã xác thực hoặc đang bị thiếu. Các owner con tách kiểm tra marker hiện hữu, marker bị thiếu và cổng cảnh báo một lần mỗi process.

The public functions keep their previous paths and signatures. Existing integration coverage remains in `core_attestation.rs`; nearest-owner tests are under each behavior folder. Source-only refactor work does not claim executable tests have run.

Hai hàm public giữ nguyên đường dẫn và chữ ký cũ. Test integration hiện có vẫn ở `core_attestation.rs`; test gần owner nằm trong folder hành vi tương ứng. Refactor chỉ rà source không được tính là đã chạy test.
