# package_names

EN: Exposes the sorted reachable Flutter SDK package-name projection used by Flutter package-config installation.

VI: Cung cấp danh sách tên package Flutter SDK đã sắp xếp cho luồng cài package-config Flutter.

- Input/output: project root and selected SDK root → reachable SDK package names.
- Invariant: uses the same full closure validation as manifest expansion; no independent or weaker scan is introduced.
- Errors: propagates the closure's typed Flutter errors unchanged.
- Extension: keep name projection here; do not bypass SDK-root or manifest validation.
- Tests: `../test/sdk_dependency_graph.rs` verifies the existing projection assertion; runtime test remains pending.
