# package_adapter

## Purpose / Mục đích

Owns manifest identity, parse, list and PackageAdapter capability delegation. / Owns manifest identity, parse, list and PackageAdapter capability delegation.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into the adapter.
- [package_adapter.rs](package_adapter.rs) contains the implementation for this responsibility.
- Errors reuse `mgc_types::MgError`/`MgResult`; this owner defines no private error taxonomy.

## Tests / Kiểm thử

Manifest and installed-state boundaries are exercised in `adapters/game/tests/integration.rs`. Tests stay at the crate consumer boundary or in the existing framework-specific test owner; no empty test directory is created.
