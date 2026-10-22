# Shared error API owner

This folder exposes the existing `MgError` taxonomy and `MgResult<T>` alias. `mod.rs` keeps the public paths stable and only wires the private `error` and `mg_result` owners. The shared enum remains cohesive; domain-specific error classification and retry policy remain with the functional owners.

Thư mục này xuất taxonomy `MgError` và alias `MgResult<T>` hiện có. `mod.rs` giữ ổn định public path và chỉ nối hai owner private `error` và `mg_result`. Enum dùng chung vẫn cohesive; phân loại lỗi theo domain và retry policy thuộc owner chức năng.
