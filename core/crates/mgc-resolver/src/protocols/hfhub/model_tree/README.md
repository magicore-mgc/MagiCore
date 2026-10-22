# Model tree owner

Converts API tree file entries into locked ModelFile values, skips non-file entries, validates the same path/blob/LFS fields, constructs direct pinned-revision URLs, and sorts by path. Local typed failures map to the legacy MgError::Other diagnostics. HTTP status and JSON decoding remain in resolve_model.

Chuyển entry file của API tree thành ModelFile đã lock, bỏ qua entry không phải file, kiểm tra cùng field path/blob/LFS, tạo URL trực tiếp theo revision ghim và sắp xếp theo path. Lỗi typed cục bộ ánh xạ về diagnostic MgError::Other cũ. HTTP status và parse JSON vẫn thuộc resolve_model.
