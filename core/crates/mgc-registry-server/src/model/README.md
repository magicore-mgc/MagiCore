# Registry models / Model registry

This folder preserves the public `mgc_registry_server::model::*` surface while separating the wire contracts by responsibility: npm package metadata, npm version metadata, search responses, OCI payloads, PyPI files, the registry error response, and the custom deprecation deserializer. The root `mod.rs` only wires modules and re-exports the original public type names.

Serde derives, field visibility/types, rename/default/skip rules, and constructors were moved without behavior changes. Focused tests live beside each owner. Existing registry route/storage/test callers stay on the same `crate::model::*` path.

Thư mục này giữ nguyên bề mặt `mgc_registry_server::model::*` công khai và tách hợp đồng dữ liệu theo trách nhiệm: metadata package/version npm, kết quả tìm kiếm, payload OCI, file PyPI, response lỗi registry và bộ deserialize deprecation. `mod.rs` gốc chỉ khai báo module và tái xuất tên kiểu cũ.

Derive Serde, visibility/kiểu field, quy tắc rename/default/skip và constructor được chuyển nguyên trạng. Test tập trung nằm cạnh từng owner. Các caller route/storage/test tiếp tục dùng đường dẫn `crate::model::*` cũ.

## Known behavior follow-up

`optional_deprecation` currently accepts string and boolean inputs but returns an empty string for any present value, so a non-empty deprecation reason is lost. The structural batch preserves and records this behavior; it is not evidence that the representation is correct. See the S487 batch and source review before making a separate behavior change.

`optional_deprecation` hiện nhận chuỗi và boolean nhưng trả chuỗi rỗng với mọi giá trị hiện diện, làm mất nội dung lý do deprecation. Batch cấu trúc này giữ nguyên và ghi nhận hành vi đó; điều này không khẳng định biểu diễn hiện tại là đúng. Xem batch S487 và source review trước khi sửa hành vi riêng.
