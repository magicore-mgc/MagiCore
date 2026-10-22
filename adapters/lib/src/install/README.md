# Lib install owners

`mod.rs` is the compatibility facade for the Lib adapter's install lifecycle. It keeps the current `crate::install::*` call paths and cross-lane test registration. `install/install/` owns coordinator dispatch, offline policy, native preflight, and the empty-graph lock transaction while preserving each lane's existing install and canonical-lock publication order.

The responsibility owners are:

- `cargo_lock/`: safe empty `Cargo.lock` projection and atomic replacement.
- `lock_coverage/`: ecosystem mapping, protocol-marker reattachment, package matching, and pre-install coverage checks.
- `lock_read/`: MGC lock V3/V4 loading, V4 projection, validation, and reuse of existing rows.
- `lock_write/`: owner-scoped V3/V4 lock merge and atomic publication.
- `native_install_context/`: resolver package conversion and shared CAS construction.
- `*_native_install/`: Go, Maven, NuGet, Rust, and Python native lanes. Each lane owns `error.rs` for its local typed boundary and maps back into the established shared `MgError` categories.
- `python_runtime/`: Python lock ownership, manifest checks, wheel verification, runtime paths, and installed-package reporting. Leaf owners own typed `error.rs` files; the outward facade retains existing `MgError` messages and categories.
- `cargo_compat/`, `fetch.rs`, `shared_store.rs`, `verify.rs`: existing Cargo projection, transport, shared-store, and verification owners.
- `install/install/`: coordinator plus separate offline-policy, native-preflight, and empty-graph transaction owners; details and constraints are in its local `README.md`.
- `test/`: cross-lane lifecycle and lock publication/validation/schema tests; `mod.rs` wires modules and `support.rs` owns shared fixtures. Python runtime tests live under the nearest `python_runtime/` owner. Rust/Python installer cache and importability tests live under their nearest native owner `test/` folders. Go/Maven/NuGet currently have no owner-local install-behavior tests.

## Chủ sở hữu install Lib

`mod.rs` là facade tương thích cho lifecycle install của adapter Lib. File này giữ nguyên đường gọi `crate::install::*` và đăng ký test cross-lane. Folder `install/install/` sở hữu coordinator, offline policy, native preflight và transaction graph rỗng; thứ tự install và chính sách ghi lock của từng lane được giữ nguyên.

Các owner theo trách nhiệm:

- `cargo_lock/`: tạo projection `Cargo.lock` rỗng an toàn và thay file atomic.
- `lock_coverage/`: ánh xạ ecosystem, gắn marker protocol, so khớp package và kiểm tra lock trước install.
- `lock_read/`: đọc lock MGC V3/V4, projection V4, xác minh và tái sử dụng row có sẵn.
- `lock_write/`: merge lock V3/V4 theo owner và ghi atomic.
- `native_install_context/`: chuyển package resolver và dựng CAS dùng chung.
- `*_native_install/`: lane native Go, Maven, NuGet, Rust và Python. Mỗi lane sở hữu `error.rs` cho ranh giới lỗi typed cục bộ và ánh xạ về nhóm `MgError` dùng chung.
- `python_runtime/`: ownership lock Python, kiểm tra manifest, xác minh wheel, đường dẫn runtime và liệt kê package. Leaf owner có `error.rs` typed; facade giữ nguyên nội dung và nhóm `MgError` hiện hành.
- `cargo_compat/`, `fetch.rs`, `shared_store.rs`, `verify.rs`: các owner hiện có cho Cargo projection, tải, shared store và xác minh.
- `install/install/`: coordinator và owner riêng cho offline-policy, native-preflight và transaction graph rỗng; xem README gần module để biết thứ tự và giới hạn.
- `test/`: test lifecycle nhiều lane và publication/validation/schema lock; `mod.rs` chỉ wiring module, `support.rs` giữ fixture chung. Test Python runtime nằm dưới owner gần nhất trong `python_runtime/`. Test cache/importability của Rust/Python nằm trong `test/` gần owner native. Go/Maven/NuGet hiện chưa có test hành vi install riêng tại owner.
