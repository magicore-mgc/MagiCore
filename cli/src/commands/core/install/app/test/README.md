# App install tests

These tests cover the install-only rule that package arguments must use `mgc add-app`; its typed diagnostic is owned by this folder's `error.rs`. After the structural pass, run `cargo test -p mgc --features app flutter_install_with_package_arguments_requires_add`. The source is currently NOT_RUN.

# Test install App

Test kiểm tra quy tắc riêng của install: package arguments phải dùng `mgc add-app`; diagnostic typed thuộc `error.rs` trong folder này. Sau structural pass, chạy `cargo test -p mgc --features app flutter_install_with_package_arguments_requires_add`. Source hiện NOT_RUN.
