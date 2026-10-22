# sdk_package_manifest

EN: Resolves one package root inside the selected Flutter SDK and validates its regular `pubspec.yaml` identity and supported metadata.

VI: Tìm root package trong Flutter SDK đã chọn, rồi kiểm tra file `pubspec.yaml` thường, danh tính và metadata được hỗ trợ.

- Input/output: canonical SDK root and validated SDK package name → dependency mapping or no dependencies.
- Invariants: package root remains within the selected SDK; symlink/non-regular manifest files, identity mismatch, overrides/workspace and malformed dependency mappings fail closed.
- Errors: existing typed Flutter manifest errors retain their diagnostic boundary.
- Extension: package-location policy remains in `sdk/package_root`; metadata-source policy stays here.
- Tests: malformed metadata and override cases are under `../test/`; no tests were run for S274.
