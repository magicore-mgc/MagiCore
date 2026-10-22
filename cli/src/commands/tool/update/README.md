# update

## Purpose / Mục đích

Owns the managed tool update transaction, target refresh and rollback to the previous record/shim. / Owns the managed tool update transaction, target refresh and rollback to the previous record/shim.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into `commands::tool`.
- [update.rs](update.rs) contains the implementation for this responsibility.
- Errors keep the existing `anyhow::Result`/`crate::error` behavior; no local error taxonomy was added.

## Tests / Kiểm thử

Update and rollback routes are covered by `cli/tests/global_tool_e2e.rs`. Existing test source is kept byte-identically; executable verification is deferred until the repository-wide production-source pass closes.
