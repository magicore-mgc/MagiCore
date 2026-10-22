# json_file

## Purpose / Mục đích

Owns regular-file JSON reads and create-new private file writes, including Unix no-follow/0600 behavior. / Owns regular-file JSON reads and create-new private file writes, including Unix no-follow/0600 behavior.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into `commands::tool`.
- [json_file.rs](json_file.rs) contains the implementation for this responsibility.
- Errors keep the existing `anyhow::Result`/`crate::error` behavior; no local error taxonomy was added.

## Tests / Kiểm thử

File safety is exercised by the global-tool E2E suite. Existing test source is kept byte-identically; executable verification is deferred until the repository-wide production-source pass closes.
