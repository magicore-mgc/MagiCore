# Scaffold flags compatibility facade

This same-name owner preserves the existing `commands::core::scaffold_flags::{ScaffoldFlags, ContainerFlags}` type paths. New code should import `WebScaffoldFlags` from the Web owner and `ContainerFlags` from the shared-core owner. The facade contains only re-exports and owns no parsing or runtime behavior.

## Tiếng Việt

Owner cùng tên này giữ tương thích đường dẫn kiểu cũ `commands::core::scaffold_flags::{ScaffoldFlags, ContainerFlags}`. Code mới nên import `WebScaffoldFlags` từ owner Web và `ContainerFlags` từ owner shared-core. Facade chỉ export lại type, không sở hữu parser hay runtime behavior.
