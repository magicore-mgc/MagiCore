# Lockfile schema / Schema lockfile

Owns the stable `mgc_lockfile::schema::*` facade and the v3 serializable models. `mod.rs` only declares owners and re-exports the previous public paths. Each owner keeps its schema behavior, local contract tests and focused README; crate-wide `LockfileError` remains the shared error contract.

## Trách nhiệm

Sở hữu facade ổn định `mgc_lockfile::schema::*` và các model serialize schema v3. `mod.rs` chỉ khai báo owner và export lại public path cũ. Mỗi owner giữ hành vi schema, test contract gần nhất và README riêng; `LockfileError` cấp crate vẫn là hợp đồng lỗi dùng chung.
