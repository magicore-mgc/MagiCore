# scaffold_provider

## Purpose / Mục đích

Owns the scaffold capability probe and its existing claim. / Owns the scaffold capability probe and its existing claim.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into the adapter.
- [scaffold_provider.rs](scaffold_provider.rs) contains the implementation for this responsibility.
- Errors reuse `mgc_types::MgError`/`MgResult`; this owner defines no private error taxonomy.

## Tests / Kiểm thử

Framework scaffold generation has dedicated tests under `src/scaffold/test/`. Tests stay at the crate consumer boundary or in the existing framework-specific test owner; no empty test directory is created.
