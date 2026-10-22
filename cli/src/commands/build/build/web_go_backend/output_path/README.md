# Go build output path / Đường dẫn output build Go

## Responsibility / Trách nhiệm

Creates `.magicore/build/go` one component at a time and refuses symlink/non-directory components. It also validates the target executable path before the compiler writes it.

Tạo `.magicore/build/go` từng thành phần và từ chối symlink/thành phần không phải directory. Đồng thời kiểm tra đường dẫn executable đích trước khi compiler ghi.

## Errors and tests / Lỗi và test

Unsafe paths use the established build-output diagnostic; filesystem failures retain their source. Existing symlink-negative coverage is registered in `cli/src/test/build_test.rs`. A local typed error adapter and colocated test owner remain future work.

Đường dẫn không an toàn dùng diagnostic build-output hiện có; lỗi filesystem giữ source. Test âm symlink đã đăng ký ở `cli/src/test/build_test.rs`. Adapter lỗi typed và test owner cùng folder là việc sau.
