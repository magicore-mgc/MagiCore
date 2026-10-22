# Lockfile root model / Model root lockfile

Owns the v3 `Lockfile`, `LockfileMetadata`, `SignerInfo` and root-level construction/package-query methods. It composes the package, ownership and workspace models without changing their serialized field names or defaults.

## Trách nhiệm

Sở hữu `Lockfile`, `LockfileMetadata`, `SignerInfo` schema v3 và các hàm tạo/truy vấn package ở root. Owner ghép model package, ownership, workspace mà không đổi tên field serialize hay giá trị mặc định.
