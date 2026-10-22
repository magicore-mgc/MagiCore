# Lock-patch owner

Owns `LockPatch`, the persisted lockfile record. It stores the package name/version, resulting SHA-256 string and supplied timestamp without validation or patch application.

Sở hữu `LockPatch`, record được lưu trong lockfile. Nó giữ tên/version package, chuỗi SHA-256 kết quả và timestamp đầu vào mà không xác thực hay áp dụng patch.

The nearest test verifies the existing serde round trip. This owner has no error path, so it has no local error module.

Test gần owner xác minh serde round trip hiện có. Owner không có nhánh lỗi nên không cần module error riêng.
