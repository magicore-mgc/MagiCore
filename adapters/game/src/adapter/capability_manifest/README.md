# capability_manifest

## Purpose / Mục đích

Owns the static list of capabilities actually claimed by GameAdapter. / Owns the static list of capabilities actually claimed by GameAdapter.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into the adapter.
- [capability_manifest.rs](capability_manifest.rs) contains the implementation for this responsibility.
- Errors reuse `mgc_types::MgError`/`MgResult`; this owner defines no private error taxonomy.

## Tests / Kiểm thử

Capability truth is exercised by the public-adapter integration tests. Tests stay at the crate consumer boundary or in the existing framework-specific test owner; no empty test directory is created.
