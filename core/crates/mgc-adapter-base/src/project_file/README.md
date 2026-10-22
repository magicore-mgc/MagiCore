# Project-file security owners

This folder preserves the shared API for no-follow project reads and atomic manifest writes. The mod.rs file only wires modules and exports the API plus typed ProjectFileError. Child owners isolate policy/filesystem errors, regular-file reads, atomic replacement, platform-specific path checks, and Unix dirfd operations.

Thư mục này giữ API dùng chung để đọc project không theo symlink và ghi manifest atomic. File mod.rs chỉ nối module/export API cùng ProjectFileError typed. Các owner con tách lỗi policy/filesystem, đọc file thường, thay file atomic, kiểm tra path theo OS và thao tác dirfd Unix.

The existing public integration tests remain at core/crates/mgc-adapter-base/tests/project_file_security.rs. Errors keep their existing Display text but now surface as MgError::Anyhow so consumers can downcast ProjectFileError and inspect the original source. No test result is claimed until executable verification runs.

Các integration test public hiện có vẫn ở core/crates/mgc-adapter-base/tests/project_file_security.rs. Lỗi giữ nguyên Display nhưng trả qua MgError::Anyhow để caller downcast ProjectFileError và xem source gốc. Test riêng của owner đặt trong test/ gần phần triển khai; chưa tuyên bố test pass trước khi chạy xác minh thực tế.
