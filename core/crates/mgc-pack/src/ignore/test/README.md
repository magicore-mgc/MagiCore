# File-selection integration tests

This directory contains the registered Cargo integration target for `mgc_pack::ignore::select_files`. It intentionally tests the public module route and full filesystem behavior, rather than reaching private helper internals.

Run target: `cargo test -p mgc-pack --test test_ignore` after the repository storage gate allows runtime verification.

READMEVN: Đây là integration test Cargo đã đăng ký cho API công khai `mgc_pack::ignore::select_files`. Test giữ đường gọi public và hành vi filesystem hoàn chỉnh; chưa chạy trong source-first phase.
