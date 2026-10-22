# factory

## Purpose / Mục đích

Owns the stable adapter_for constructor and engine detection handoff. / Owns the stable adapter_for constructor and engine detection handoff.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into the adapter.
- [factory.rs](factory.rs) contains the implementation for this responsibility.
- Errors reuse `mgc_types::MgError`/`MgResult`; this owner defines no private error taxonomy.

## Tests / Kiểm thử

The exported factory path is covered by `adapters/game/tests/integration.rs`. Tests stay at the crate consumer boundary or in the existing framework-specific test owner; no empty test directory is created.
