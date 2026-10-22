# Cas Generation Error

## Responsibility

Define errors shared by CAS generation and project lock operations.

## Trách nhiệm

Định nghĩa lỗi dùng chung cho thao tác CAS generation và khóa project.

## Layout

- `cas_generation_error.rs`: implementation owned by this folder.
- `mod.rs`: module wiring and the smallest required export surface.
- Registered integration tests remain at the crate's Cargo test paths and exercise this public database contract.

## Boundaries

Keep SQL statements, transaction ordering, lock scope, result ordering, feature gates, and error text unchanged. This owner has no local test folder because its existing contract coverage is registered at crate level.

## Ranh giới

Giữ nguyên SQL, thứ tự transaction, phạm vi khóa, thứ tự kết quả, feature gate và lỗi. Owner không tạo test folder rỗng; các test contract sẵn có đã được đăng ký ở cấp crate.
