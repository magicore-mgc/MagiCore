# Package-file index / Chỉ mục file package

This module owns the rebuildable MessagePack acceleration layer over SQLite's package-file table. It keeps the public `StoreIndex` API and delegates serialization, rebuild, upsert, and CAS pruning to focused owners.

Module này quản lý lớp tăng tốc MessagePack có thể dựng lại từ bảng file package trong SQLite. Module giữ API `StoreIndex` và giao serialize, rebuild, upsert, prune CAS cho các owner chuyên trách.

The index is not the source of truth: SQLite remains authoritative. Do not change cache format, write ordering, corruption fallback, or prune safety policy in a structure-only change.

Index không phải nguồn dữ liệu chuẩn: SQLite vẫn là nguồn chính. Không đổi format cache, thứ tự ghi, fallback khi hỏng hoặc chính sách an toàn prune trong refactor cấu trúc.
