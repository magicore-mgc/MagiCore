# Package Scope

Owns ecosystem package scoping and package/repository name validation.

**Owned API:** `scoped_package, core_scoped_oci_repository, canonical_pypi_name`. Imports are shared through the local trusted prelude; public names re-export to preserve `crate::trusted::*`.

# Package Scope — Tiếng Việt

Sở hữu namespace package theo hệ sinh thái và kiểm tra tên package/repository.

**API sở hữu:** `scoped_package, core_scoped_oci_repository, canonical_pypi_name`. Import dùng chung qua prelude nội bộ; tên công khai được export để giữ nguyên `crate::trusted::*`.
