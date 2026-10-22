# Connection

## Responsibility

Keep raw SQLite access inside the store crate for the StoreIndex scan. Downstream production crates use explicit Database operations; integration fixtures may access the connection only with the `test-util` feature.

## Trách nhiệm

Giữ truy cập SQLite thô bên trong crate store cho lượt quét StoreIndex. Crate production downstream dùng thao tác Database tường minh; fixture integration chỉ được truy cập connection khi bật feature `test-util`.

## Layout

- `connection.rs`: implementation owned by this folder.
- `mod.rs`: module wiring and the smallest required export surface.
- `conn()` is crate-private; `conn_for_tests()` is feature-gated and hidden from normal production builds.
- Registered integration tests remain at the crate's Cargo test paths and use the explicit test-only connection where fixture SQL is required.

## Boundaries

Keep SQL statements, transaction ordering, lock scope, result ordering, feature gates, and error text unchanged. This owner has no local test folder because its connection-visibility and fixture contract coverage is registered at crate level.

## Ranh giới

Giữ nguyên SQL, thứ tự transaction, phạm vi khóa, thứ tự kết quả, feature gate và lỗi. Owner không tạo test folder rỗng; contract về visibility connection và fixture được test tại cấp crate.
