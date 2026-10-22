# App integration test owners

This folder splits the App adapter integration target by behavior. The Cargo target remains at
tests/integration.rs, and the root harness retains the original registrations.

Thư mục này tách integration test App adapter theo hành vi. Cargo target vẫn nằm tại
tests/integration.rs; harness gốc giữ các đăng ký ban đầu.

- language_detection/: framework markers, identity, selection and symlink safety.
- adapter_contract/: AppLanguage and project-ownership contracts.
- lock_replay/: explicit non-SDK requirement preservation.
- lifecycle/: parse, resolve/install, refusal and listing.
- audit_sbom/: audit and SBOM.
- support/: shared temporary project directory fixture.
