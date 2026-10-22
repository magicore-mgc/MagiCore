# record_lookup

## Purpose / Mục đích

Owns unique binary-to-managed-record lookup and rejects duplicate/unsafe tool entries. / Owns unique binary-to-managed-record lookup and rejects duplicate/unsafe tool entries.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into `commands::tool`.
- [record_lookup.rs](record_lookup.rs) contains the implementation for this responsibility.
- Errors keep the existing `anyhow::Result`/`crate::error` behavior; no local error taxonomy was added.

## Tests / Kiểm thử

Lookup behavior is exercised by update/remove/list commands and the global-tool E2E suite. Existing test source is kept byte-identically; executable verification is deferred until the repository-wide production-source pass closes.
