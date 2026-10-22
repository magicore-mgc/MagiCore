# install

## Purpose / Mục đích

Owns the user-visible install transaction and recovery-marker sequence. / Owns the user-visible install transaction and recovery-marker sequence.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into `commands::tool`.
- [install.rs](install.rs) contains the implementation for this responsibility.
- Errors keep the existing `anyhow::Result`/`crate::error` behavior; no local error taxonomy was added.

## Tests / Kiểm thử

Install and recovery behavior is covered by `cli/src/commands/test/tool.rs` and `cli/tests/global_tool_e2e.rs`. Existing test source is kept byte-identically; executable verification is deferred until the repository-wide production-source pass closes.
