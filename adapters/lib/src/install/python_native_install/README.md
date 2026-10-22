# Native Python install

Owns the MGC-managed PyPI install path, offline cache replay, and importability preflight.

## Chủ sở hữu

Sở hữu luồng install PyPI do MGC quản lý, replay cache offline và preflight khả năng import.

Cache verification and importability-preflight tests are registered under `test/` and share fixtures from `install/test/support.rs`.

Test xác minh cache và preflight khả năng import được đăng ký trong `test/`, dùng fixture chung ở `install/test/support.rs`.

`error.rs` owns offline-cache and materialization errors while preserving existing shared protocol/store categories.

`error.rs` sở hữu lỗi cache offline và materialize, đồng thời giữ nguyên loại lỗi protocol/store dùng chung.
