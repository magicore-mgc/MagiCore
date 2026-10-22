# Package cache streaming target / Target đọc package cache dạng luồng

This Cargo auto-discovered integration target checks file-backed cache reads, misses and final-component symlink rejection. Test bodies live in same-name behavior folders.

Target integration tự discovery của Cargo kiểm tra đọc cache dạng file, cache miss và từ chối symlink ở thành phần path cuối. Thân test nằm trong folder behavior cùng tên.

Runtime command: `cargo test -p mgc-store --test package_cache_streaming`. This is documented for the runtime-verification phase; it was not run during source-first work.

Lệnh runtime: `cargo test -p mgc-store --test package_cache_streaming`. Lệnh dành cho phase xác minh runtime; chưa chạy trong giai đoạn source-first.
