# remove

## Purpose / Mục đích

Owns removal of a tool only after verifying its MagiCore-owned shim. / Owns removal of a tool only after verifying its MagiCore-owned shim.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into `commands::tool`.
- [remove.rs](remove.rs) contains the implementation for this responsibility.
- Errors keep the existing `anyhow::Result`/`crate::error` behavior; no local error taxonomy was added.

## Tests / Kiểm thử

Removal and collision safety are covered by `cli/tests/global_tool_e2e.rs`. Existing test source is kept byte-identically; executable verification is deferred until the repository-wide production-source pass closes.
