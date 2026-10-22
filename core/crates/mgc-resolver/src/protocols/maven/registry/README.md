# Maven registry / Registry Maven

This folder owns the native Maven repository engine. `registry.rs` keeps type construction/configuration; `coordinate_validation/`, `repository_transport/`, `parent_version_resolution/`, and `artifact_materialization/` own independent behavior. Shared `MgError`/`MgResult` errors preserve the existing API contract.

Folder này quản lý engine repository Maven native. `registry.rs` giữ type và cấu hình; các owner còn lại tách validate tọa độ, HTTP, resolve qua POM cha và ghi local layout. Lỗi dùng chung `MgError`/`MgResult` giữ nguyên hợp đồng hiện tại.
