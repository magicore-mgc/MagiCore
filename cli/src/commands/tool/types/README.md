# types

## Purpose / Mục đích

Owns the private JSON schemas and filenames for installed/recovering tool records. / Owns the private JSON schemas and filenames for installed/recovering tool records.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into `commands::tool`.
- [types.rs](types.rs) contains the implementation for this responsibility.
- Errors keep the existing `anyhow::Result`/`crate::error` behavior; no local error taxonomy was added.

## Tests / Kiểm thử

Record behavior is asserted by the nearest command unit tests and the global-tool E2E suite. Existing test source is kept byte-identically; executable verification is deferred until the repository-wide production-source pass closes.
