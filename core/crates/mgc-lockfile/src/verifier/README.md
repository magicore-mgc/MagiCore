# Lockfile verifier API / API xác minh lockfile

This folder preserves `mgc_lockfile::verifier::*` and separates status values, legacy sidecar verification, v4 inline verification, the tamper projection, provenance completeness, and console messages. `mod.rs` only wires/re-exports. S293 found no production caller; S294 now wires the existing v3 policy into shared `serialize_lockfile`, pending source review and runtime verification. The v3 API does not validate v4 source-table identity.

Thư mục này giữ `mgc_lockfile::verifier::*` và tách status, xác minh sidecar legacy, xác minh inline v4, phép chiếu tamper, completeness provenance và message console. `mod.rs` chỉ nối/export. S293 chưa tìm thấy caller production; S294 đã nối policy v3 hiện có vào `serialize_lockfile` dùng chung, đang chờ rà source và kiểm chứng runtime. API v3 này không xác minh identity trong bảng source của v4.

Read [ProgressReport.md](docs/ProgressReport.md) for source-only status. Tests are authored but NOT_RUN until the repository-wide source pass closes.
