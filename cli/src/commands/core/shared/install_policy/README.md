# Shared install policy / Policy install dùng chung

This folder groups shared install-policy decisions. `install_policy.rs` wires the child policies; the minimum-release-age decision and lock refresh live in `age_gate/`. Add a child owner only when there is a distinct policy with its own behavior and evidence.

Folder này gom các quyết định policy install dùng chung. `install_policy.rs` nối các policy con; quyết định minimum-release-age và refresh lock nằm trong `age_gate/`. Chỉ thêm owner con khi có policy riêng với behavior và bằng chứng riêng.
