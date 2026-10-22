# managed_roots

## Purpose / Mục đích

Owns the fixed user-scoped data/tools/bin paths, real-directory checks, permissions, deterministic project key and shim path. / Owns the fixed user-scoped data/tools/bin paths, real-directory checks, permissions, deterministic project key and shim path.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into `commands::tool`.
- [managed_roots.rs](managed_roots.rs) contains the implementation for this responsibility.
- Errors keep the existing `anyhow::Result`/`crate::error` behavior; no local error taxonomy was added.

## Tests / Kiểm thử

Root and path behavior is covered by the global-tool E2E suite. Existing test source is kept byte-identically; executable verification is deferred until the repository-wide production-source pass closes.
