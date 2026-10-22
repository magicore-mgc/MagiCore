# Release build scripts / Script build bản phát hành

This folder owns matrix-driven CLI build/package operations, release artifact-name selection, and platform-specific CycloneDX generation called by release-build.yml. The workflow owns the runner, target matrix, toolchain/action setup, step conditions, and artifact upload. Scripts consume only explicit MGC_RELEASE_* environment values declared by their workflow steps.

Folder này sở hữu build/package CLI theo matrix, chọn tên artifact và tạo SBOM CycloneDX theo hệ điều hành, được release-build.yml gọi. Workflow sở hữu runner, ma trận target, setup toolchain/action, điều kiện step và upload artifact. Script chỉ nhận MGC_RELEASE_* được khai báo tường minh tại workflow step.

`build_standard_unix.sh` owns native Unix builds and records the Go/cgo GOARCH/CGO_ENABLED constraint. `build_cross_unix.sh` owns cross builds. `build_windows.ps1` owns native Windows builds. `package_unix.sh` and `package_windows.ps1` create the two archives and SHA-256 sidecars. `set_artifact_names.sh` and the Unix/Windows SBOM scripts retain their focused responsibilities.

`build_standard_unix.sh` sở hữu native Unix build và ghi rõ constraint Go/cgo GOARCH/CGO_ENABLED; `build_cross_unix.sh` sở hữu cross build; `build_windows.ps1` sở hữu Windows native build; `package_unix.sh` và `package_windows.ps1` tạo hai archive và SHA-256 sidecar. `set_artifact_names.sh` cùng hai script SBOM giữ trách nhiệm riêng.

Validated version and matrix values are passed through step environment variables, never interpolated into script source. Scripts run from the repository root and fail on command errors. Add nearest tests for build command selection, archive names/checksums, artifact-name output and SBOM command construction in the global test phase; no tests are created or run in this source-first pass.
