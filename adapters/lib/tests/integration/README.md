# Lib integration test owners

This folder splits the Lib adapter integration target by behavior. The Cargo target remains
tests/integration.rs, and the root harness retains every original test registration.

Thư mục này tách integration test Lib adapter theo hành vi. Cargo target vẫn ở tests/integration.rs;
harness gốc giữ nguyên các đăng ký test.

- language_detection/: language markers and symlink safety.
- capability_contract/: capability truth and core identity.
- python_ownership/: pip refusal and foreign Python lock ownership.
- manifest_io/: Rust/Python manifest round trips and TS workspace filtering.
- lifecycle/: mutation refusal and installed-package listing.
- audit_sbom/: scanner truth and SBOM.
- support/: shared temporary project directory.
