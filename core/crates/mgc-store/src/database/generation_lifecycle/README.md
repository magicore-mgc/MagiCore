# Generation Lifecycle

## Responsibility

Allocate, promote and abort CAS install generations under SQLite transactions.

## Trách nhiệm

Cấp phát, promote và abort generation cài CAS trong transaction SQLite.

## Layout

- `generation_lifecycle.rs`: allocates a unique generation token.
- `promote/`: validates, retires and promotes the token atomically.
- `abort/`: validates and aborts only the token's staging claims.
- `mod.rs`: module wiring and the smallest required export surface.
- Registered integration tests remain at the crate's Cargo test paths and exercise this public database contract.

## Boundaries

Keep SQL statements, transaction ordering, lock scope, result ordering, feature gates, and error text unchanged. Nearest-owner test allocation remains pending for the repository test-organization phase; the current crate-level contract tests stay registered and unchanged during structural extraction.

## Ranh giới

Giữ nguyên SQL, thứ tự transaction, phạm vi khóa, thứ tự kết quả, feature gate và lỗi. Việc đặt test gần owner nhất còn chờ phase tổ chức test toàn repo; test contract cấp crate được giữ nguyên trong lượt tách cấu trúc.
