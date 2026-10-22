# CAS claim-row insertion

Owns the prepared insert and per-hash error mapping for a validated claim batch.

## Trách nhiệm

Quản lý prepared insert và ánh xạ lỗi theo từng hash của batch claim đã xác minh.

The caller owns token validation, the writer gate, and transaction lifetime. Preserve order, SQL, and diagnostics.

Caller sở hữu bước xác minh token, writer gate và vòng đời transaction. Giữ nguyên thứ tự, SQL và thông báo lỗi.
