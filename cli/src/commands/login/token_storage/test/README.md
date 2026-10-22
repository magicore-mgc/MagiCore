# Token storage tests

EN: Covers project-versus-user `.npmrc` target selection and the missing-home error without touching a real home directory.

VI: Kiểm tra chọn đích `.npmrc` project/user và lỗi thiếu home mà không sửa home directory thật.

Actual file creation, permissions and symlink defenses remain covered by the shared `mgc-config` tests and must be run in the later verification phase.

Tạo file thật, permissions và chặn symlink thuộc test `mgc-config` dùng chung và phải chạy ở phase xác minh sau.
