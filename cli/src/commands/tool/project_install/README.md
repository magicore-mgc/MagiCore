# project_install

## Purpose / Mục đích

Owns the isolated package project, registry config sanitization, native install handoff and executable target validation. / Owns the isolated package project, registry config sanitization, native install handoff and executable target validation.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into `commands::tool`.
- [project_install.rs](project_install.rs) contains the implementation for this responsibility.
- Errors keep the existing `anyhow::Result`/`crate::error` behavior; no local error taxonomy was added.

## Tests / Kiểm thử

Package layout and identity are covered by the global-tool E2E suite. Existing test source is kept byte-identically; executable verification is deferred until the repository-wide production-source pass closes.
