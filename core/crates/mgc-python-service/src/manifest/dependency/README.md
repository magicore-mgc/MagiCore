# Python dependency conversion

`dependency.rs` is the single grammar boundary between simple PEP 508 strings and shared `DependencySpec` values. It also serializes those values back into TOML strings without broadening the accepted syntax.

Extras, environment markers, direct URL references, compound constraints, and invalid/empty requirements remain errors. The local `error.rs` classifies syntax rejection and enters `MgError::Anyhow` so callers can retain the local type. Shared package-name and version-range failures keep their existing error variants; the public result stays `MgResult`. Both the ownership predicate and project parser call this owner to avoid accepting a requirement that the parser would flatten differently.

## Trách nhiệm

`dependency.rs` là ranh giới grammar giữa chuỗi PEP 508 đơn giản và `DependencySpec` dùng chung. Module cũng chuyển giá trị đó về chuỗi TOML mà không mở rộng cú pháp được chấp nhận.

Extras, environment marker, URL trực tiếp, constraint ghép và requirement rỗng/sai vẫn trả lỗi. `error.rs` phân loại lỗi cú pháp và dùng `MgError::Anyhow` để caller giữ local type. Lỗi package-name/version-range dùng lại variant hiện có; kiểu kết quả ngoài vẫn `MgResult`. Predicate ownership và parser project cùng gọi owner này để không nhận requirement khác grammar khi đọc manifest.
