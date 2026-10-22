# Lib install test owners

This folder contains the unit tests registered by `adapters/lib/src/install/mod.rs`. `mod.rs` only declares test modules. `support.rs` owns shared fixtures and re-exports the private install symbols/types needed by these child test modules. Keep each behavior test in the smallest responsibility file below.

| File | Test responsibility |
|---|---|
| `install_lifecycle.rs` | Existing delta-lock completion lifecycle |
| `lock_publication.rs` | Signed-lock preservation, direct roots, atomic publication, digest representation |
| `lock_owner_merge.rs` | Ecosystem-scoped lock merge and stale-row handling |
| `lock_validation.rs` | Lock coverage, malformed/symlinked/oversized lock rejection |
| `lock_schema.rs` | V4 lock projection, ordering, and lossy-row rejection |

Python runtime tests live under their nearest `python_runtime/` owners: `path_entries/`, `manifest_guard/`, `lock_owner/`, `wheel_materialization/`, and `python_runtime/test/`.

Empty/offline graph behavior now lives under `install/install/empty_graph/test/`. Offline allowlist tests live under `install/install/offline_policy/test/`, and hostile-lock preflight tests live under `install/install/preflight/test/`.

## Chủ sở hữu test install Lib

Folder này chứa unit test được đăng ký trong `adapters/lib/src/install/mod.rs`. `mod.rs` chỉ khai báo module test. `support.rs` sở hữu fixture chung và re-export symbol/type install private mà các test con cần. Mỗi behavior test nằm trong file trách nhiệm hẹp nhất bên dưới.

| File | Trách nhiệm test |
|---|---|
| `install_lifecycle.rs` | Lifecycle hoàn thành delta-lock hiện có |
| `lock_publication.rs` | Giữ signed lock, direct root, publication atomic, biểu diễn digest |
| `lock_owner_merge.rs` | Merge lock theo ecosystem và xử lý row cũ |
| `lock_validation.rs` | Coverage lock, từ chối lock lỗi/symlink/quá cỡ |
| `lock_schema.rs` | Projection lock V4, thứ tự và từ chối row gây mất dữ liệu |

Hành vi graph rỗng/offline nằm tại `install/install/empty_graph/test/`. Test allowlist offline nằm tại `install/install/offline_policy/test/`; test preflight lock lỗi nằm tại `install/install/preflight/test/`. Test Python runtime nằm trong owner gần nhất dưới `python_runtime/`: `path_entries/`, `manifest_guard/`, `lock_owner/`, `wheel_materialization/` và `python_runtime/test/`.

The baseline contains 68 test items: 21 shared coordinator/lock tests, 31 Cargo-compat tests, and 16 Rust/Python native/runtime tests. S528 moves two of the shared cases to the empty-graph owner without changing their assertions and adds four focused tests, for 72 total. All remain `NOT_RUN` until the repository-wide source pass closes.

Baseline có 68 test item: 21 test coordinator/lock dùng chung, 31 test Cargo-compat và 16 test native/runtime Rust/Python. S528 chuyển hai test trong nhóm dùng chung tới owner graph rỗng mà giữ nguyên assertion, đồng thời thêm bốn test tập trung, tổng cộng 72. Tất cả vẫn `NOT_RUN` tới khi source pass toàn repo đóng.
