# identity

## Purpose / Mục đích

Owns CoreIdent and the public engine-detection/engine-name methods. / Owns CoreIdent and the public engine-detection/engine-name methods.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into the adapter.
- [identity.rs](identity.rs) contains the implementation for this responsibility.
- Errors reuse `mgc_types::MgError`/`MgResult`; this owner defines no private error taxonomy.

## Tests / Kiểm thử

Detection and engine access are covered by the public integration tests. Tests stay at the crate consumer boundary or in the existing framework-specific test owner; no empty test directory is created.
