# shim

## Purpose / Mục đích

Owns executable containment, Unix symlink/Windows command shim creation and ownership-checked removal. / Owns executable containment, Unix symlink/Windows command shim creation and ownership-checked removal.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into `commands::tool`.
- [shim.rs](shim.rs) contains the implementation for this responsibility.
- Errors keep the existing `anyhow::Result`/`crate::error` behavior; no local error taxonomy was added.

## Tests / Kiểm thử

Windows shim rendering and target validation are tested in `cli/src/commands/test/tool.rs`; ownership is exercised by global-tool E2E cases. Existing test source is kept byte-identically; executable verification is deferred until the repository-wide production-source pass closes.
