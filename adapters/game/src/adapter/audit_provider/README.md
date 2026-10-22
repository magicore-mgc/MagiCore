# audit_provider

## Purpose / Mục đích

Owns the existing shared polyglot audit delegation. / Owns the existing shared polyglot audit delegation.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into the adapter.
- [audit_provider.rs](audit_provider.rs) contains the implementation for this responsibility.
- Errors reuse `mgc_types::MgError`/`MgResult`; this owner defines no private error taxonomy.

## Tests / Kiểm thử

Public audit delegation is exercised from `adapters/game/tests/integration.rs`. Tests stay at the crate consumer boundary or in the existing framework-specific test owner; no empty test directory is created.
