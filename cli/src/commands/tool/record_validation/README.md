# record_validation

## Purpose / Mục đích

Owns identity/schema/path validation for persisted records and recovery markers; rejects unsupported or path-escaping values. / Owns identity/schema/path validation for persisted records and recovery markers; rejects unsupported or path-escaping values.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into `commands::tool`.
- [record_validation.rs](record_validation.rs) contains the implementation for this responsibility.
- Errors keep the existing `anyhow::Result`/`crate::error` behavior; no local error taxonomy was added.

## Tests / Kiểm thử

Portable target validation has nearest tests in `cli/src/commands/test/tool.rs`; malformed records are covered by command E2E tests. Existing test source is kept byte-identically; executable verification is deferred until the repository-wide production-source pass closes.
