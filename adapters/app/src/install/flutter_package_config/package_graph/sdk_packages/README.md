# Flutter SDK package projection

Adds only the selected, canonical Flutter SDK packages to the generated Dart graph. `sdk_manifest.rs` owns SDK-root lookup and identity checks; `sdk_dependencies.rs` accepts only dependency constraint forms the Dart graph can represent. Shared graph errors stay at the parent package-graph boundary.

## Trách nhiệm

Chỉ thêm package Flutter SDK được chọn và canonical vào Dart graph. `sdk_manifest.rs` sở hữu tìm package trong SDK và kiểm tra identity; `sdk_dependencies.rs` chỉ nhận dạng constraint mà Dart graph biểu diễn được. Lỗi chung dùng boundary package graph cha.
