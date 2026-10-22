# declared_dependencies

EN: Reads only direct `sdk: flutter` entries from the project `pubspec.yaml` and merges runtime/dev declarations.

VI: Chỉ đọc các mục trực tiếp `sdk: flutter` trong `pubspec.yaml` và gộp khai báo runtime/dev.

- Input/output: project root → sorted `(package name, dev-only)` pairs.
- Invariant: runtime wins when the same SDK package appears in both groups; package names pass the existing Flutter SDK validator.
- Errors: uses the Flutter manifest boundary `FlutterError`/`FlutterResult`; no local error variant is introduced.
- Extension: add declared Flutter SDK metadata handling here; transitive SDK traversal belongs to `dependency_closure`.
- Tests: parent `../test/` covers the complete SDK dependency graph; no standalone runtime result is claimed in S274.
