# Python runtime owners

Owns the Lib adapter's Python lock ownership helpers, verified runtime path resolution, and installed-package listing. The public runtime path APIs are re-exported by `install/mod.rs`; adapter, CLI, Cargo compatibility, and source-test imports continue through that facade.

## Owners

- `python_runtime.rs`: PEP 503 normalization, project owner, foreign-manifest guard, lock-owner selection, and safe site-directory naming.
- `python_runtime.rs` is a compatibility facade over `mgc-python-service`; that service owns errors for the delegated helpers.
- `path_entries/`: lock-backed wheel/runtime path verification and public path APIs.
- `installed_packages/`: reports only packages whose locked materialization has been verified.
- `lock_owner/error.rs`, `manifest_guard/error.rs`, `wheel_materialization/error.rs`, `installed_packages/error.rs`: own typed failures for each leaf operation and map to the existing shared `MgError` category at the facade boundary.

Behavior tests are registered beside the owner they exercise: path resolution, manifest guarding, lock ownership, wheel materialization, or project ownership. Each owner test folder has its own README and imports shared fixtures from `install/test/support.rs`.

## Chủ sở hữu

Sở hữu helper chọn core của lock Python, resolve đường dẫn runtime đã xác minh và liệt kê package đã cài của Lib. API runtime được export qua `install/mod.rs`; đường gọi hiện tại của adapter, CLI, Cargo compatibility và test source giữ nguyên.

- `python_runtime.rs`: chuẩn hóa PEP 503, core sở hữu project, chặn manifest ngoài phạm vi, chọn owner lock và tạo tên thư mục site an toàn.
- `python_runtime.rs` là facade tương thích gọi `mgc-python-service`; service sở hữu lỗi của các helper được ủy quyền.
- `path_entries/`: xác minh đường dẫn runtime/wheel theo lock và API đường dẫn.
- `installed_packages/`: chỉ báo package có materialization đã xác minh.
- `error.rs` tại từng leaf owner: giữ variant lỗi typed theo trách nhiệm rồi chuyển về nhóm `MgError` hiện hành ở facade; source chain của một số thông báo cũ vẫn bị làm phẳng khi đi qua `MgError::Other/Integrity`.

Test hành vi được đăng ký gần owner: resolve path, manifest guard, lock owner, materialize wheel hoặc project ownership. Mỗi folder test có README riêng và dùng fixture chung từ `install/test/support.rs`.
