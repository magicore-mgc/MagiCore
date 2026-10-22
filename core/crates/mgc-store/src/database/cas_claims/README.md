# Cas Claims

## Responsibility

Register generation-scoped CAS blob claims and expose test-only destructive helpers.

## Trách nhiệm

Ghi claim blob CAS theo generation và giữ helper xóa phá hủy chỉ dành cho test.

## Layout

- `cas_claims.rs`: generation-scoped claim writes and their atomic token validation.
- `claim_token_validation/`: verifies the project-scoped staging token before writes.
- `claim_rows_insert/`: prepares and writes the claim batch under the active transaction.
- `maintenance/`: live-reference listing and test-only destructive maintenance.
- `mod.rs`: module wiring and the smallest required export surface.
- Registered integration tests remain at the crate's Cargo test paths and exercise this public database contract.

## Boundaries

Keep SQL statements, transaction ordering, lock scope, result ordering, feature gates, and error text unchanged. This owner has no local test folder because its existing contract coverage is registered at crate level.

## Ranh giới

Giữ nguyên SQL, thứ tự transaction, phạm vi khóa, thứ tự kết quả, feature gate và lỗi. Owner không tạo test folder rỗng; các test contract sẵn có đã được đăng ký ở cấp crate.
