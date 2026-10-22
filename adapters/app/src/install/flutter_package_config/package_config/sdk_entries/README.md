# Flutter SDK entries

Builds Dart package-config rows for packages supplied by the active Flutter SDK. It validates package manifests, identity, and the `lib` directory while sharing the duplicate-name set with registry entries.

`error.rs` classifies missing SDK roots, registry conflicts, unsafe/non-regular package files, malformed manifests, and URI failures. Existing cases are under `test/sdk_entries.rs`.

## Chủ sở hữu

Dựng row package-config Dart cho package từ Flutter SDK đang dùng. Owner kiểm tra manifest, danh tính và thư mục `lib`, dùng chung tập tên trùng với entry registry.

`error.rs` phân loại thiếu SDK root, xung đột registry, file không an toàn/không regular, manifest lỗi và URI lỗi. Các case hiện có nằm tại `test/sdk_entries.rs`.
