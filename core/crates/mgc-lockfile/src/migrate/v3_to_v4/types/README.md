# Shared conversion types / Kiểu dùng chung cho chuyển đổi

Defines the small `SourceCatalog` DTO used by the registry-source and package-conversion owners. It has no independent behavior; those nearest owners exercise its fields and invariants.

Định nghĩa DTO `SourceCatalog` nhỏ dùng bởi owner catalog registry và chuyển đổi package. DTO không có hành vi độc lập; test tại hai owner gần nhất kiểm tra các field và bất biến.

`mod.rs` only wires and re-exports the DTO within the migration module.

`mod.rs` chỉ nối module và re-export DTO trong phạm vi migration.
