# AI integration test owners

This folder splits the existing AI adapter integration target by behavior. The parent Cargo target remains tests/integration.rs, and its test names and attributes remain registered there.

Thư mục này tách integration test AI adapter theo hành vi. Cargo target gốc vẫn là tests/integration.rs; tên và thuộc tính đăng ký test vẫn nằm ở harness gốc.

- project_detection/: marker and symlink-safe detection.
- adapter_contract/: framework metadata and adapter contracts.
- python_lane/: Python package ownership and lifecycle refusal/resolution.
- audit_sbom/: model-artifact audit and SBOM.
- support/: shared temporary project directory helper.
