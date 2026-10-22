# Project Install Lock

## Responsibility

Own the Database constructor and public export for the cross-process project install lock; the handle lifecycle lives in a nested owner.

## Trách nhiệm

Quản lý constructor Database và public export cho khóa install project liên process; vòng đời handle nằm trong owner con.

## Layout

- `project_install_lock.rs`: `Database::project_install_lock` constructor.
- `project_install_lock_handle/`: lock-file handle, try-lock acquisition and drop behavior.
- `mod.rs`: module wiring and the smallest required export surface.
- Registered integration tests remain at the crate's Cargo test paths and exercise this public database contract.

## Boundaries

Keep SQL statements, transaction ordering, lock scope, result ordering, feature gates, and error text unchanged. This owner has no local test folder because its existing contract coverage is registered at crate level.

## Ranh giới

Giữ nguyên SQL, thứ tự transaction, phạm vi khóa, thứ tự kết quả, feature gate và lỗi. Owner không tạo test folder rỗng; các test contract sẵn có đã được đăng ký ở cấp crate.
