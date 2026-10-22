# Lease Column Probe

Owns `lease_columns_present` used by the lease-column migration coordinator.

## Trách nhiệm

Quản lý `lease_columns_present` được coordinator migration cột lease sử dụng.

Keep the existing SQLite probe/ALTER semantics, error mapping, and callers unchanged.

Giữ nguyên cách probe/ALTER SQLite, ánh xạ lỗi và caller hiện tại.
