# `mgc-store` integration tests

This directory contains Cargo integration-test targets for store, CAS and cache contracts. Target names and paths are defined in `../Cargo.toml`; keep each path stable when reorganizing test bodies. The compiled-cache integrity target keeps its registrations in `compiled_cache_integrity.rs` and groups implementation under `compiled_cache_integrity/`.

Thư mục này chứa các Cargo integration-test target cho hợp đồng store, CAS và cache. Tên target và đường dẫn được khai báo trong `../Cargo.toml`; phải giữ path ổn định khi tổ chức lại phần thân test. Target integrity compiled-cache giữ đăng ký trong `compiled_cache_integrity.rs` và nhóm implementation dưới `compiled_cache_integrity/`.

When the runtime-verification phase is authorized, run `cargo test -p mgc-store --test test_compiled_cache_integrity`. This command is documented only; it has not been executed as part of the source-first pass.

Khi đến phase xác minh runtime, chạy `cargo test -p mgc-store --test test_compiled_cache_integrity`. Lệnh chỉ được ghi tài liệu; chưa được chạy trong phase source-first.

The generation-token target also keeps its Cargo entry at `generation_token.rs`; the root registers 24 tests and delegates implementations to behavior owners under `generation_token/`. It includes direct SQL checks that fresh and migrated schemas reject orphan CAS claims and that the read-only integrity API detects corrupted fixture state. Raw connection access is enabled only by the crate's `test-util` feature. Its runtime command is `cargo test -p mgc-store --test test_generation_token`; it is documented for the later verification phase and was not run during this source-first refactor.

Target generation-token cũng giữ entry Cargo ở `generation_token.rs`; file gốc đăng ký 24 test và chuyển implementation vào các owner theo hành vi dưới `generation_token/`. Suite có kiểm tra SQL trực tiếp rằng schema mới và schema đã migration từ chối CAS claim mồ côi, đồng thời API integrity chỉ đọc phát hiện fixture bị lỗi. Raw connection chỉ được mở khi bật feature `test-util`. Lệnh runtime là `cargo test -p mgc-store --test test_generation_token`; chỉ ghi để chạy ở phase xác minh sau, chưa chạy trong đợt refactor source-first này.

The package-cache concurrency target keeps its root at `package_cache_concurrency.rs`; an include-only test owner groups the two existing test bodies without changing registration names or order. Its runtime command is `cargo test -p mgc-store --test package_cache_concurrency`, documented for the later verification phase and not run during source-first work.

Target concurrency của package cache giữ file root `package_cache_concurrency.rs`; owner test chỉ include hai thân test hiện có, không đổi tên hay thứ tự đăng ký. Lệnh runtime là `cargo test -p mgc-store --test package_cache_concurrency`, ghi cho phase xác minh sau và chưa chạy trong giai đoạn source-first.

The `package_cache_path_security.rs` target checks metadata/tarball path containment, collision-resistant cache keys, ordinary-path compatibility and recursive tarball counting. Cargo auto-discovers it; test bodies live under same-name behavior folders. Its runtime command is `cargo test -p mgc-store --test package_cache_path_security`, documented for the later verification phase and not run during source-first work.

Target `package_cache_path_security.rs` kiểm tra containment path metadata/tarball, cache key chống va chạm, tương thích path thường và đếm tarball đệ quy. Cargo tự discovery target; thân test nằm trong folder behavior cùng tên. Lệnh runtime là `cargo test -p mgc-store --test package_cache_path_security`, ghi cho phase xác minh sau và chưa chạy trong giai đoạn source-first.

The `package_cache_streaming.rs` target checks file-backed cache reads, absent entries and final-symlink rejection. Its runtime command is `cargo test -p mgc-store --test package_cache_streaming`; it is pending runtime verification.

Target `package_cache_streaming.rs` kiểm tra đọc cache dạng file, entry chưa tồn tại và từ chối symlink cuối. Lệnh runtime là `cargo test -p mgc-store --test package_cache_streaming`; đang chờ xác minh runtime.
