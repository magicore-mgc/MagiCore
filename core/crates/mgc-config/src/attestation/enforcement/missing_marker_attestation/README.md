# Missing-marker attestation / Chứng thực marker bị thiếu

Checks trust state when the project core marker is absent, including renamed or removed markers. Warn mode keeps legacy discovery; Require blocks missing or mismatched identity.

Kiểm tra trust khi thiếu core marker của project, kể cả marker bị đổi tên hoặc xóa. Warn giữ khả năng nhận diện project cũ; Require chặn identity bị thiếu hoặc không khớp.

The owner keeps its typed failure variants and diagnostics local. Existing integration cases remain in `core_attestation.rs`; focused error contract tests are adjacent.

Owner giữ các variant lỗi typed và diagnostic tại chỗ. Các integration case hiện có vẫn nằm trong `core_attestation.rs`; test contract lỗi được đặt cạnh implementation.
