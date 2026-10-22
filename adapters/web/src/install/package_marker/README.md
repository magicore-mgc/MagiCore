# Package marker owners / Các owner marker package

This module validates the identity and content of Web packages shared in the extracted-package cache. `mod.rs` is the compatibility facade for `crate::install::package_marker::*`; keep the existing exported function names and visibility stable.

Module này xác minh định danh và nội dung package Web dùng chung trong extracted-package cache. `mod.rs` là facade tương thích cho `crate::install::package_marker::*`; giữ tên hàm và visibility đã export.

## Ownership / Phạm vi sở hữu

- `package_marker.rs` derives fast identity and full content metadata from a resolved package and tarball.
- `digest/` owns byte/path SHA-256; `tree_digest/` owns canonical tree-hash framing.
- `archive_signature/` validates gzip-tar entries and computes the normalized archive signature; `archive_signature/archive_path/` owns path rejection and root normalization.
- `executable/` projects platform metadata into the signature; `extracted_content/` recomputes a materialized tree without following symlinks.
- `marker_io/` owns `.magicore-package-root.json`; `materialized_package/` combines package identity, marker equality, and content verification.
- `ExtractedPackageMarker` and `TarballContentSignature` remain defined in `crate::cache`; moving those public types needs its own impact-reviewed compatibility slice.

## Error and compatibility contract / Lỗi và tương thích

Fallible APIs continue returning shared `MgResult`; the current `MgError::Other` variants and exact context strings are preserved. This source did not define a local typed recovery contract, so no empty or ceremonial `error.rs` is added. `atomic_write` remains the marker-write boundary.

API có thể lỗi tiếp tục trả `MgResult` dùng chung; giữ nguyên các `MgError::Other` và thông điệp context hiện tại. Source cũ không có hợp đồng recovery typed riêng, nên không tạo `error.rs` rỗng/nghi thức. `atomic_write` vẫn là boundary ghi marker.

Known behavior gaps are intentionally preserved for a later security/behavior batch: marker reads use `Path::exists`/`read_to_string`, which follows a marker symlink, and full content verification has no explicit resource budget. This structural slice does not claim to fix those conditions.

Giữ nguyên các gap hành vi để xử lý trong batch security/behavior riêng: đọc marker dùng `Path::exists`/`read_to_string`, có thể đi theo symlink; xác minh nội dung chưa có resource budget tường minh. Slice cấu trúc này không tuyên bố đã sửa các điều kiện đó.

## Tests / Kiểm thử

Each behavior owner registers separate tests under its own `test/` folder. Existing cache/install integration tests remain at their current public boundaries. Run `cargo test -p mgc-web-adapter` after the source-first phase and only when the machine storage requirement is met; the new tests are currently **NOT_RUN**.

Mỗi owner hành vi đăng ký test tách biệt tại `test/` của chính owner đó. Test tích hợp cache/install cũ giữ nguyên boundary. Chạy `cargo test -p mgc-web-adapter` sau phase source-first và chỉ khi đạt điều kiện dung lượng máy; các test mới hiện **CHƯA CHẠY**.
