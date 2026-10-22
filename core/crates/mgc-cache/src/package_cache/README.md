# Package cache / Cache package

This module owns the shared tarball cache API, integrity checks, bounded synchronization, safe paths and pruning.
Module này quản lý API cache tarball dùng chung, kiểm integrity, đồng bộ có giới hạn, path an toàn và prune.

`mod.rs` only wires owners and preserves the `mgc_cache::{PackageCache, CacheEntry}` exports. Behavior belongs to the named child owner; tests remain in the crate's Cargo integration-test area for the whole-source verification phase.
`mod.rs` chỉ nối owner và giữ export `mgc_cache::{PackageCache, CacheEntry}`. Hành vi nằm trong owner con cùng tên; test giữ ở vùng integration test của crate để chạy trong phase kiểm thử toàn source.

New entries use a BLAKE3-derived v2 path. Reads may use an old sanitized path only after containment, regular-file and integrity checks; stores write only the v2 path.
Entry mới dùng path v2 sinh từ BLAKE3. Có thể đọc path sanitize đời cũ sau khi kiểm tra nằm trong root, file thường và integrity; thao tác store chỉ ghi path v2.
