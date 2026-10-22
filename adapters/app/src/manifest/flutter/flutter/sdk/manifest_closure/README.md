# manifest_closure

EN: Owns Flutter SDK dependency-closure reading and the stable crate-local routes used by expansion and package-config installation.

VI: Quản lý việc đọc closure dependency Flutter SDK và các route nội bộ ổn định dùng bởi expansion, package-config.

## Responsibilities

- `declared_dependencies/`: parse direct `sdk: flutter` declarations and dev/runtime groups.
- `project_dependencies/`: resolve project dependency group state.
- `sdk_package_manifest/`: validate SDK package root metadata and dependency map.
- `registry_dependency/`: merge supported Pub ranges and effective dependency group.
- `dependency_closure/`: breadth-first traversal, SDK edge validation and manifest assembly.
- `package_names/`: validated SDK package-name projection for package-config installation.
- `test/`: five legacy behavior tests, relocated without changing their test bodies.

The existing `FlutterError`/`FlutterResult` contract remains the error owner; no ceremonial local error/type module is introduced. The facade exports the same three functions and the caller paths remain unchanged. No runtime test, build, formatter, CLI, E2E or CI command has been run in this source-first batch.
