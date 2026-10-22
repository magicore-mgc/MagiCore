# list

## Purpose / Mục đích

Owns deterministic listing and refusal of unsafe/unrecognized tool-store entries. / Owns deterministic listing and refusal of unsafe/unrecognized tool-store entries.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into `commands::tool`.
- [list.rs](list.rs) contains the implementation for this responsibility.
- Errors keep the existing `anyhow::Result`/`crate::error` behavior; no local error taxonomy was added.

## Tests / Kiểm thử

List behavior is covered by the global-tool E2E suite. Existing test source is kept byte-identically; executable verification is deferred until the repository-wide production-source pass closes.
