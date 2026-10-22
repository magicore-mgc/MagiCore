# Web lockfile owners / Owner lockfile Web

EN: Implements the stable `crate::lockfile` API around shared `mgc-lockfile`. Canonical format, cryptography and shared mutation rules remain in their engine owners.
VI: Triển khai API ổn định `crate::lockfile` trên engine `mgc-lockfile`; format canonical, mật mã và luật mutation dùng chung thuộc engine owner.

`mod.rs` wires/re-exports; `lockfile.rs` owns the existing write coordinator. Child owners separate cache paths, checksums, graph matching/restoration, reads, schema-specific writes, core ownership and atomic publication. `error.rs` provides local typed diagnostic categories and maps to the stable adapter `MgResult` without changing output text. `test/` holds the existing assertions split by subject and remains registered under the original test parent.

No lockfile runtime behavior is claimed verified by this source-only batch.
