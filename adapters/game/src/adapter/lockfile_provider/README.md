# lockfile_provider

## Purpose / Mục đích

Owns the Bevy Cargo manifest write and fail-closed responses for other engines. / Owns the Bevy Cargo manifest write and fail-closed responses for other engines.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into the adapter.
- [lockfile_provider.rs](lockfile_provider.rs) contains the implementation for this responsibility.
- Errors reuse `mgc_types::MgError`/`MgResult`; this owner defines no private error taxonomy.

## Tests / Kiểm thử

Game adapter engine-specific fail-closed routes are covered at the crate integration boundary. Tests stay at the crate consumer boundary or in the existing framework-specific test owner; no empty test directory is created.
