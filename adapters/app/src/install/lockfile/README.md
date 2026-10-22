# App lockfile ownership / Quyền sở hữu lockfile của App

The App lock writer updates only the rows owned by the active core and ecosystems in v3/v4 `mgc.lock`. It keeps sibling-core rows, rejects unsafe signed rewrites, and uses the schema-specific atomic publication path.

Bộ ghi lock App chỉ cập nhật row thuộc core đang chạy và ecosystem trong `mgc.lock` v3/v4. Nó giữ row của core khác, từ chối ghi đè chữ ký không an toàn và dùng đường publish atomic tương ứng schema.

## Owners / Chủ sở hữu

- `lock_write/` validates the declared ecosystem scope, resolves the project owner and dispatches to the schema lane.
- `v3_merge/` reads and merges v3 rows/root pins; it skips unchanged writes and checks the signature mutation gate before publication.
- `v4_merge/` converts lane rows losslessly, verifies the v4 base, preserves sibling rows, refuses signed rewrites and finalizes the digest.
- `owner_core/` selects the marker/config owner and canonicalizes the `cloud` alias to `clo`.
- `direct_roots/` projects direct `ResolvedGraph` package IDs in graph order.
- `atomic_publication/` runs the legacy lock mutation gate and delegates to the shared App atomic file publisher.
- `error/` owns stable local codes, operation context, retry classification and source chaining until conversion to `MgError`.
- `test/` contains shared fixtures only; behavior tests are registered under the owner they exercise.

- `lock_write/` kiểm tra ecosystem scope, tìm owner project và điều phối sang schema lane.
- `v3_merge/` đọc/gộp row và root pin v3; bỏ qua write không đổi và kiểm tra signature gate trước khi publish.
- `v4_merge/` chuyển row không mất dữ liệu, xác minh base v4, giữ sibling, từ chối ghi đè lock đã ký và chốt digest.
- `owner_core/` chọn owner từ marker/config và chuẩn hóa alias `cloud` thành `clo`.
- `direct_roots/` chiếu ID package trực tiếp từ `ResolvedGraph`, giữ thứ tự graph.
- `atomic_publication/` chạy mutation gate lock legacy rồi ủy quyền cho bộ publish file App atomic dùng chung.
- `error/` giữ mã nội bộ, operation, phân loại retry và source chain đến lúc chuyển sang `MgError`.
- `test/` chỉ chứa fixture dùng chung; test behavior được đăng ký tại owner tương ứng.

## Stable boundary and verification / Boundary ổn định và xác minh

`crate::install::{write_canonical_lock_with_roots,direct_package_ids,atomic_write_canonical_lock}` and the existing install callsites remain unchanged. The old 15 lock tests were moved without renaming; four owner tests and two typed-error tests are also present. All 21 test functions are source-only and remain NOT_RUN. The local source chain is not retained after conversion to the shared `MgError::Other(String)` contract, and no automatic retry is performed.

Các route `crate::install::{write_canonical_lock_with_roots,direct_package_ids,atomic_write_canonical_lock}` và caller install hiện có được giữ nguyên. Đã chuyển 15 test lock cũ mà không đổi tên; thêm bốn test owner và hai test typed error. Cả 21 test mới/chuyển vẫn chỉ là source, trạng thái NOT_RUN. Source chain cục bộ không còn sau khi chuyển sang hợp đồng `MgError::Other(String)` chung; không có retry tự động.
