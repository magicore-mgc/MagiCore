# Web package lifecycle

EN: Owns package hook manifest loading, script policy, bounded execution, project identity protection, and typed failures. The public `lifecycle` path stays stable; install approval and transaction orchestration remain owned by `install/`.

VI: Quản lý đọc hook từ manifest, policy script, thực thi có giới hạn, bảo vệ identity project và lỗi typed. Đường dẫn `lifecycle` công khai được giữ nguyên; phê duyệt install và điều phối transaction vẫn thuộc `install/`.

## Module map / Bản đồ module

- `mod.rs` wires modules and exports the compatibility API.
- `types.rs` owns the parsed `PackageScripts` contract shared by policy and execution.
- `manifest.rs` performs bounded, no-follow manifest loading so inspection and execution consume the same snapshot.
- `lifecycle.rs` preserves hook ordering and the guarded runner entrypoint.
- `identity_guard.rs` verifies and restores project core identity after each hook.
- `script_execution.rs` rejects package-manager wrappers, parses through `mgc-exec`, and executes only within the install scope.
- `environment.rs` owns lifecycle PATH construction and timeout resolution.
- `error.rs` owns typed failures, causal sources, and failure classification.
- `test/` contains nearest-owner tests and isolated environment mutation helpers.

## Security flow / Luồng bảo mật

The install owner applies trust and script policy first, then passes the already-inspected `PackageScripts` snapshot to `LifecycleRunner`. The runner verifies/snapshots core identity, executes preinstall → install → postinstall in a clean bounded execution scope, and checks/restores identity after every hook and at completion. A policy or execution error propagates to abort the install transaction.

Owner install áp trust và policy trước, sau đó truyền snapshot `PackageScripts` đã kiểm tra vào `LifecycleRunner`. Runner chụp identity, chạy preinstall → install → postinstall trong execution scope sạch/có giới hạn, rồi xác minh/phục hồi identity sau từng hook và khi hoàn tất. Lỗi policy hoặc thực thi được truyền lên để hủy transaction install.

Runtime tests and compilation remain pending in the explicitly deferred whole-source verification phase.
