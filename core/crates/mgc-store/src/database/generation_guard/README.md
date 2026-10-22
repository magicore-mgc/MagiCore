# Generation Guard

## Responsibility

Own the guarded `Database` constructor and re-export `CasGenerationGuard`; the RAII type and drop cleanup live in the nested type owner.

## Trách nhiệm

Quản lý constructor `Database` có guard và re-export `CasGenerationGuard`; type RAII cùng cleanup khi drop nằm trong owner type con.

## Layout

- `generation_guard.rs`: guarded database constructor.
- `cas_generation_guard/`: public guard type, disarm/accessor, debug representation and drop cleanup.
- `mod.rs`: module wiring and the smallest required export surface.
- Registered integration tests remain at the crate's Cargo test paths and exercise this public database contract.

## Boundaries

Keep SQL statements, transaction ordering, lock scope, result ordering, feature gates, and error text unchanged. This owner has no local test folder because its existing contract coverage is registered at crate level.

## Ranh giới

Giữ nguyên SQL, thứ tự transaction, phạm vi khóa, thứ tự kết quả, feature gate và lỗi. Owner không tạo test folder rỗng; các test contract sẵn có đã được đăng ký ở cấp crate.
