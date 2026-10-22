# Dependency-first workspace levels

This owner adapts the existing Kahn topological levels for workspace execution. Workspace graph edges point from a package to its workspace dependency, so execution reverses the level sequence while preserving nodes within each level. It does not change `topo_levels`, graph edges, cycle diagnostics, or each caller's concurrency policy.

The Web monorepo installer and CLI recursive executor use this API. `dependency_first_levels.rs` contains the adapter; `mod.rs` only exports it; `test/` covers ordering and error preservation.

## Tiếng Việt

Owner này chuyển các level Kahn hiện có sang thứ tự chạy workspace. Cạnh graph đi từ package tới dependency workspace nên execution đảo thứ tự các level nhưng giữ các node cùng level với nhau. Owner không đổi `topo_levels`, hướng cạnh, chẩn đoán cycle hay policy concurrency của từng caller.

Web monorepo installer và CLI recursive executor dùng API này. `dependency_first_levels.rs` chứa adapter; `mod.rs` chỉ export; `test/` kiểm tra thứ tự và việc giữ nguyên lỗi.
