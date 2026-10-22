# Cargo manifest projection owners

EN: The facade preserves the existing `cargo_compat::manifest` API while focused owners parse the project manifest, validate dependency options, resolve direct roots, and map local errors.

VI: Facade giữ API `cargo_compat::manifest` hiện tại; các owner riêng parse manifest dự án, kiểm tra option dependency, resolve root trực tiếp và ánh xạ lỗi cục bộ.

## Layout

- `mod.rs`: stable `MgResult` facade for existing callers.
- `project_manifest/`: safe Cargo.toml read and supported single-package subset.
- `dependency_specs/`: accepted dependency option policy and lock membership.
- `root_resolution/`: manifest-to-lock root identity and deterministic labels.
- `error/`: local typed errors, source-chain retention and mapping to shared `MgError`.

Each behavior owner keeps a same-name implementation, a nearest-owner test file, and a bilingual README. Existing projection tests remain under `cargo_compat/test/projection/`.
