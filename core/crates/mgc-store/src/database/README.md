# Store database

## Responsibility

This owner preserves explicit `Database` operations while separating SQLite schema/open, package rows, integrity memo, CAS integrity diagnostics, project refs, CAS token lifecycle, leases, file manifests, trust/release policy, version lookup, and RAII locks. Raw connection access is crate-private; fixture access requires `test-util`.

## Trách nhiệm

Owner này giữ các thao tác `Database` tường minh và tách schema/open SQLite, package row, integrity memo, chẩn đoán toàn vẹn CAS, project ref, CAS token lifecycle, lease, manifest file, trust/release policy, truy vấn version và khóa RAII. Connection thô chỉ dùng nội bộ; fixture cần feature `test-util`.

## Layout

- `mod.rs`: public and crate-visible facade.
- `database.rs`: owner declarations and API re-exports only.
- Each non-empty child owner has a same-name implementation, wiring facade, and bilingual README.
- Cargo-registered integration tests stay at their original paths and remain the contract tests for these owners.
- `integrity_snapshot/`: exports a typed read-only snapshot for duplicate promoted generations and orphan claims.

## Compatibility

Keep SQL text, schema versions/migration ordering, transaction boundaries, writer locks, generation claims, error variants, RAII cleanup, feature-gated test APIs, and all result ordering unchanged. `Database` fields and raw connections remain inaccessible to downstream crates; public diagnostics use explicit read-only methods.

## Tương thích

Giữ nguyên SQL, version/thứ tự migration, ranh giới transaction, writer lock, generation claim, error variant, RAII cleanup, API test có feature gate và thứ tự kết quả. Field của `Database` chỉ truy cập trong inner database module và module con; crate downstream vẫn không truy cập được.
