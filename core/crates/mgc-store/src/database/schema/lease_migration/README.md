# Lease schema migration

Owns lease-column discovery, race-safe ALTER handling, transaction-local fail-closed revalidation, and post-open compatibility checks.

## Migration schema lease

Quản lý dò cột lease, xử lý ALTER an toàn trước race, kiểm tra lại fail-closed trong transaction và xác thực sau khi mở database.
