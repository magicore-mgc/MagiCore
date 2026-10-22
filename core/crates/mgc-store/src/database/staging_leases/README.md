# Staging Leases

## Responsibility

List staging leases and identify claim sets already covered by promoted generations.

## Trách nhiệm

Liệt kê staging lease và nhận diện claim đã được generation promoted bảo vệ.

## Layout

- `staging_leases.rs`: base staging-lease query.
- `covered_by_promoted/`: query for staging claims covered by promoted generations.
- `mod.rs`: module wiring and the smallest required export surface.
- Registered integration tests remain at the crate's Cargo test paths and exercise this public database contract.

## Boundaries

Keep SQL statements, transaction ordering, lock scope, result ordering, feature gates, and error text unchanged. This owner has no local test folder because its existing contract coverage is registered at crate level.

## Ranh giới

Giữ nguyên SQL, thứ tự transaction, phạm vi khóa, thứ tự kết quả, feature gate và lỗi. Owner không tạo test folder rỗng; các test contract sẵn có đã được đăng ký ở cấp crate.
