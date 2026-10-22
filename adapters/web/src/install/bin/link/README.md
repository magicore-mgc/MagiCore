# Platform bin link / Link bin theo nền tảng

Creates Unix symlinks or the existing non-Unix `.cmd` shim that invokes `node`. It does not run the shim or install Node. Existing error categories and message templates remain in force.

Tạo symlink trên Unix hoặc `.cmd` shim non-Unix hiện có để gọi `node`. Owner không chạy shim và không cài Node. Category lỗi và template message hiện có được giữ nguyên.

Platform-specific tests are pending for the later test phase; this batch does not claim Windows execution was verified. Run the crate tests on Unix and Windows CI when that phase is authorized; no test/build command ran here.

Test theo nền tảng đang chờ phase test sau; batch này không tuyên bố đã xác minh chạy trên Windows. Chạy crate test ở CI Unix và Windows khi tới phase đó; batch này không chạy test/build.
