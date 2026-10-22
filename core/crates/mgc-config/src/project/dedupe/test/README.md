# Dedupe settings tests

This folder tests only the generic dedupe DTO: the omitted-field default, serialized field spelling, and explicit `true` roundtrip. Project-level configuration loading remains covered by `mgc-config/tests/project.rs`.

# Test cấu hình dedupe

Folder này chỉ kiểm tra DTO dedupe dùng chung: default khi thiếu field, tên field khi serialize và roundtrip giá trị `true`. Luồng load cấu hình project vẫn được bao phủ bởi `mgc-config/tests/project.rs`.
