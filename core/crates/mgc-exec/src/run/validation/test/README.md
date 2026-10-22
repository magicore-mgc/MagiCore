# Validation tests / Test validation

These tests exercise the validator directly, without launching a child process. Unix-only cases cover normal and dangling symlinks; the Windows-only case verifies drive-qualified paths are rejected without a project boundary.

Các test gọi trực tiếp validator, không chạy process con. Case Unix kiểm tra symlink thường và symlink hỏng; case Windows kiểm tra đường dẫn có drive bị từ chối khi thiếu ranh giới dự án.

`mod.rs` only includes `validation.rs` so the test module keeps its original parent scope.
`mod.rs` chỉ include `validation.rs` để giữ nguyên scope cha của test module.
