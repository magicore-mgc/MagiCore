# record_write

## Purpose / Mục đích

Owns serialization and atomic publication of the tool record and pending recovery marker. / Owns serialization and atomic publication of the tool record and pending recovery marker.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into `commands::tool`.
- [record_write.rs](record_write.rs) contains the implementation for this responsibility.
- Errors keep the existing `anyhow::Result`/`crate::error` behavior; no local error taxonomy was added.

## Tests / Kiểm thử

Persistence behavior is covered by install/update recovery paths in `cli/tests/global_tool_e2e.rs`. Existing test source is kept byte-identically; executable verification is deferred until the repository-wide production-source pass closes.
