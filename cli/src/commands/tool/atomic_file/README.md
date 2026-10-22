# atomic_file

## Purpose / Mục đích

Owns temp-file replacement, Windows backup/restore and Unix parent-directory sync. / Owns temp-file replacement, Windows backup/restore and Unix parent-directory sync.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into `commands::tool`.
- [atomic_file.rs](atomic_file.rs) contains the implementation for this responsibility.
- Errors keep the existing `anyhow::Result`/`crate::error` behavior; no local error taxonomy was added.

## Tests / Kiểm thử

Atomic rollback behavior is covered by update and record paths in the global-tool E2E suite. Existing test source is kept byte-identically; executable verification is deferred until the repository-wide production-source pass closes.
