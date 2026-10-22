# native_install

## Purpose / Mục đích

Owns delegation to the MagiCore Web native install lane, capability gate and temporary credential copy/removal. / Owns delegation to the MagiCore Web native install lane, capability gate and temporary credential copy/removal.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into `commands::tool`.
- [native_install.rs](native_install.rs) contains the implementation for this responsibility.
- Errors keep the existing `anyhow::Result`/`crate::error` behavior; no local error taxonomy was added.

## Tests / Kiểm thử

Native lifecycle safety is covered by the global-tool E2E suite. Existing test source is kept byte-identically; executable verification is deferred until the repository-wide production-source pass closes.
