# Attestation message and digest / Thông điệp và digest attestation

Owns the existing domain-separated canonical message and BLAKE3 hex conversion used by project config and identity-slot hashing. It delegates hashing to `mgc_crypto` and returns the same lowercase hex representation.

Quản lý canonical message tách domain và chuyển BLAKE3 sang hex dùng cho config project/identity slot. Owner gọi `mgc_crypto` và giữ dạng hex chữ thường như cũ.

The owner introduces no new error type or policy. Its nearest test pins exact message bytes and digest shape.

Owner không thêm error type hay policy mới. Test gần nhất khóa chính xác message và cấu trúc digest.
