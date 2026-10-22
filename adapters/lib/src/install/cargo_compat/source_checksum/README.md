# source_checksum

## Responsibility

This owner creates and verifies Cargo directory-source checksum records from an archive already verified by MagiCore. It checks archive SHA-256, derives the expected file map from the verified archive, hashes the extracted source without following symbolic links, handles filesystem case aliases by canonical path identity, and writes the checksum record atomically.

## Owners

- validation: SHA-256 input shape.
- tree_hash: safe walk and hashing of regular extracted files.
- canonical_file_map: canonical containment and case-alias reconciliation.
- write_record: generate Cargo checksum JSON.
- verify_record: validate checksum JSON and compare locked archive with extracted files.

The shared MgError Integrity/Other variants remain the boundary. S394 preserves their category and message; no owner-specific retry or recovery classifier is present to justify another wrapper type.

## Stable API

cargo_compat continues to expose write_source_checksum and verify_source_checksum at the same crate-private paths. The validation helper is re-exported only to the sibling package_graph owner that already calls it.

## Safety invariants

Reject a source root that is not a regular directory, symbolic links, special files, non-UTF-8 paths, path escapes, archive/source digest mismatch and conflicting case aliases. Do not skip an extracted file, alter archive member selection, change the checksum JSON shape or bypass atomic_write.

## Tests

Unit test modules are registered beside each behavior owner. Existing checksum lifecycle test bodies remain unchanged. Tests were authored/moved but have not been executed in the source-first phase.

## Ghi chú tiếng Việt

Owner này tạo và xác minh checksum Cargo directory-source từ archive đã được MagiCore xác minh. Nó kiểm SHA-256 archive, dựng file map từ archive tin cậy, hash source giải nén mà không theo symbolic link, xử lý case alias bằng canonical path identity và ghi checksum nguyên tử.

MgError Integrity/Other dùng chung tiếp tục là boundary. S394 giữ nguyên category/message; chưa có retry/recovery classifier riêng để cần bọc thêm một type lỗi. Hai API write_source_checksum và verify_source_checksum giữ nguyên đường dẫn crate-private. Validator chỉ export nội bộ tới package_graph vốn đã gọi nó.

Các invariant an toàn: từ chối source root không phải thư mục thường, symlink, special file, path không UTF-8, path thoát root, digest archive/source lệch và case alias xung đột. Không bỏ file, đổi cách chọn archive member, đổi JSON checksum hoặc bypass atomic_write.

Test được đăng ký gần từng behavior owner. Ba body test lifecycle checksum cũ giữ nguyên; test đã viết/chuyển nhưng chưa chạy trong phase source-first.
