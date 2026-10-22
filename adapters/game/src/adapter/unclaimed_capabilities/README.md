# unclaimed_capabilities

## Purpose / Mục đích

Keeps unclaimed capability marker implementations adjacent and visibly fail-closed. / Keeps unclaimed capability marker implementations adjacent and visibly fail-closed.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into the adapter.
- [unclaimed_capabilities.rs](unclaimed_capabilities.rs) contains the implementation for this responsibility.
- Errors reuse `mgc_types::MgError`/`MgResult`; this owner defines no private error taxonomy.

## Tests / Kiểm thử

Capability manifest and unsupported operations are checked through the public integration boundary. Tests stay at the crate consumer boundary or in the existing framework-specific test owner; no empty test directory is created.
