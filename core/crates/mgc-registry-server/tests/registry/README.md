# Registry integration test owners / Owner integration test Registry

## Purpose / Mục đích

This folder owns the integration tests declared by the existing Cargo target at `tests/registry.rs`. The root file retains crate-level lint configuration and imports; this directory groups test items by behavior. It does not implement Registry production behavior or add another test target.

Thư mục này sở hữu các integration test của Cargo target hiện có tại `tests/registry.rs`. File root giữ lint cấp crate và import; thư mục này nhóm test theo hành vi. Đây không phải production code và không tạo thêm test target.

## Module map / Sơ đồ module

| File | Coverage |
| --- | --- |
| `auth_service.rs` | Auth construction, admin token, account persistence and registration safeguards |
| `model_storage.rs` | Package serialization, store initialization and audit log |
| `access_policy.rs` | Role/scope authorization and cross-package/repository write restrictions |
| `npm_routes.rs` | npm route matching, parameter handling, delete and OCI tag catalog |
| `oci_publish.rs` | OCI manifest validation, verified/chunked uploads and range rejection |
| `oci_routes.rs` | OCI route matching |
| `pypi_upload.rs` | PyPI upload, index and download behavior |
| `scope_glob.rs` | Scope glob matching |
| `mod.rs` | Private module wiring; imports the existing target's test support names |

All moved test items retain their original attributes, comments and bodies. Runtime execution remains pending; a source-only move does not establish that the integration tests compile or pass.

Mọi test item được chuyển giữ nguyên attribute, comment và body gốc. Runtime vẫn chờ; chỉ chuyển source không chứng minh integration test compile hoặc pass.
