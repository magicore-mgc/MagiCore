# Dependency resolver owner

This owner keeps the `DependencyResolver` implementation as a stable facade. `probe.rs` owns capability preflight, `resolve.rs` owns native/Web lane order and Java fail-closed routing, `lock_staging.rs` owns pending lock replacement, and `error.rs` owns resolver-local lock state errors. Native protocol construction is shared with `resolve_first` through `adapter/native_registry/`.

Nearest tests are in `test/`; existing real-lane coverage remains under `adapters/lib/tests/`. The resolver uses registry protocols and never introduces an external package-manager process.

## Phạm vi

Folder này giữ implementation `DependencyResolver` làm facade ổn định. `probe.rs` sở hữu preflight capability; `resolve.rs` sở hữu thứ tự Web/native và nhánh Java fail-closed; `lock_staging.rs` sở hữu thay pending lock; `error.rs` sở hữu lỗi trạng thái lock cục bộ. Dựng protocol native được dùng chung với `resolve_first` qua `adapter/native_registry/`.

Test gần owner nằm trong `test/`; kiểm thử lane thật hiện có được giữ tại `adapters/lib/tests/`. Resolver đi qua registry protocol và không thêm tiến trình package manager bên ngoài.
