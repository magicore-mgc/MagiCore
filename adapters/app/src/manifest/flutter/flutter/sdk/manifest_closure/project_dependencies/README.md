# project_dependencies

EN: Builds the project's package-to-runtime/dev lookup used while merging Flutter SDK transitive dependencies.

VI: Tạo bảng tra package runtime/dev của dự án khi gộp dependency bắc cầu từ Flutter SDK.

- Input/output: project root → ordered map where `true` means dev-only.
- Invariant: a package present in runtime and dev sections is classified runtime.
- Errors: YAML/file errors stay in the existing Flutter manifest error boundary.
- Extension: project manifest grouping rules live here; SDK package metadata is owned by `sdk_package_manifest`.
- Tests: `../test/runtime_dev_promotion.rs` checks the cross-owner promotion behavior; runtime execution remains pending.
