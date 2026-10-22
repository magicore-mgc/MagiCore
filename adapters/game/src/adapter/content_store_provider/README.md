# content_store_provider

## Purpose / Mục đích

Owns the explicit unsupported content-store/install responses. / Owns the explicit unsupported content-store/install responses.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into the adapter.
- [content_store_provider.rs](content_store_provider.rs) contains the implementation for this responsibility.
- Errors reuse `mgc_types::MgError`/`MgResult`; this owner defines no private error taxonomy.

## Tests / Kiểm thử

The consumer-facing fail-closed install assertion is in `adapters/game/tests/integration.rs`. Tests stay at the crate consumer boundary or in the existing framework-specific test owner; no empty test directory is created.
