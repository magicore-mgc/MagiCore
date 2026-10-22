# Staging Lease

## Responsibility

Describe a staging CAS generation and its process/time/claim evidence.

## Trách nhiệm

Mô tả CAS staging generation cùng bằng chứng process/thời gian/claim.

## Layout

- `staging_lease.rs`: implementation owned by this folder.
- `mod.rs`: module wiring and the smallest required export surface.
- Registered integration tests remain at the crate's Cargo test paths and exercise this public database contract.

## Boundaries

Keep SQL statements, transaction ordering, lock scope, result ordering, feature gates, and error text unchanged. This owner has no local test folder because its existing contract coverage is registered at crate level.

## Ranh giới

Giữ nguyên SQL, thứ tự transaction, phạm vi khóa, thứ tự kết quả, feature gate và lỗi. Owner không tạo test folder rỗng; các test contract sẵn có đã được đăng ký ở cấp crate.
