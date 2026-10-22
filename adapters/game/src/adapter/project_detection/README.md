# project_detection

## Purpose / Mục đích

Owns ProjectDetector dispatch to the existing game manifest detector. / Owns ProjectDetector dispatch to the existing game manifest detector.

## Implementation / Triển khai

- [Module entry](mod.rs) wires this owner into the adapter.
- [project_detection.rs](project_detection.rs) contains the implementation for this responsibility.
- Errors reuse `mgc_types::MgError`/`MgResult`; this owner defines no private error taxonomy.

## Tests / Kiểm thử

Public detection cases remain in `adapters/game/tests/integration.rs`. Tests stay at the crate consumer boundary or in the existing framework-specific test owner; no empty test directory is created.
