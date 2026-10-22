# Lock settings owner

This owner stores the serialized `[lock]` policy, writer-lock timeout values, and new-lock schema target. It contains no lock acquisition or signature verification logic; those behaviors remain with their lock and trust consumers. The parent project module re-exports `LockConfig` at its existing path.

All fields remain optional and omitted when `None`. This DTO has no validation or recovery logic, so parse errors remain owned by project loading and no local error type is added. Matching serde tests live in `test/`.

# Owner cấu hình lock

Owner này lưu policy `[lock]`, timeout writer-lock và schema đích cho lock mới. Nó không chứa logic lấy lock hay verify chữ ký; các hành vi đó thuộc lock/trust consumer. Module project re-export `LockConfig` ở public path hiện tại.

Mọi field vẫn là tùy chọn và bị bỏ khi `None`. DTO không có logic validate/recovery, vì vậy lỗi parse thuộc project loader và không tạo error type local. Test serde khớp với DTO nằm trong `test/`.
