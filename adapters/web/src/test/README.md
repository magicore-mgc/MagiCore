# Web adapter tests

This folder owns unit-test sources for the Web adapter. The crate test harness remains registered from `src/lib.rs` through `unit_tests/`; subject owners keep test behavior near its test module without creating separate Cargo targets.

Thư mục này quản lý mã nguồn unit test của Web adapter. Harness của crate vẫn được đăng ký từ `src/lib.rs` qua `unit_tests/`; các owner nhóm test theo chủ đề và không tạo Cargo target riêng.

- `unit_tests/` — shared fixtures, imports and registrations.
- `resolution_tests/` — package-resolution behavior, split by subject.
- Root-level `*_tests.rs` files — focused owners retained while their cases remain independently understandable.
