# CI/CD adapter integration tests

This folder keeps one stable Cargo integration target while assigning test behavior to small owners. The root integration.rs harness preserves test names, order, attributes, and async/sync registration.

Thư mục này giữ một Cargo integration target ổn định và chia hành vi test theo owner nhỏ. Harness integration.rs ở thư mục tests giữ nguyên tên, thứ tự, attribute và đăng ký async/sync.

- provider_detection/: marker and config selection.
- provider_model/: provider string representation.
- adapter_discovery/: adapter construction and marker detection.
- adapter_contract/: adapter metadata.
- package_operations/: fail-closed package operations.
- audit_sbom/: audit and SBOM.
- support/: shared temporary project helper.
