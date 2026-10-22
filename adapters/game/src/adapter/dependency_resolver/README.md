# dependency_resolver

## Purpose / Mục đích

Owns add/remove/update behavior and preserves explicit unsupported results. / Owns add/remove/update behavior and preserves explicit unsupported results.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into the adapter.
- [dependency_resolver.rs](dependency_resolver.rs) contains the implementation for this responsibility.
- Errors reuse `mgc_types::MgError`/`MgResult`; this owner defines no private error taxonomy.

## Tests / Kiểm thử

No-external-package-manager and unsupported-operation cases are in `adapters/game/tests/integration.rs`. Tests stay at the crate consumer boundary or in the existing framework-specific test owner; no empty test directory is created.
