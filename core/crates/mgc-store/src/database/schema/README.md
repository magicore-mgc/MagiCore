# Schema

## Responsibility

Open SQLite stores, apply ordered schema migrations and configure WAL/lease columns.

## Trách nhiệm

Mở SQLite store, áp dụng migration theo thứ tự và cấu hình WAL/cột lease.

## Layout

- `schema.rs`: `Database::open` orchestration only.
- `base_schema/`: initial table DDL.
- `generation_migration/`: legacy CAS generation-token migration.
- `generation_sequence/`: monotonic generation counter bootstrap.
- `lease_migration/`: lease-column probes, ALTER handling and revalidation.
- `wal_mode/`: bounded WAL initialization retry.
- `mod.rs`: module wiring and the smallest required export surface.
- Registered integration tests remain at the crate's Cargo test paths and exercise this public database contract.

## Boundaries

Keep SQL statements, transaction ordering, lock scope, result ordering, feature gates, and error text unchanged. Nearest-owner test allocation remains pending for the repository test-organization phase; the current crate-level migration tests stay registered and unchanged during structural extraction.

## Ranh giới

Giữ nguyên SQL, thứ tự transaction, phạm vi khóa, thứ tự kết quả, feature gate và lỗi. Việc đặt test gần owner nhất còn chờ phase tổ chức test toàn repo; test migration cấp crate được giữ nguyên trong lượt tách cấu trúc.
