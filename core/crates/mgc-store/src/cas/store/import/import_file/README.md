# File import owner

Owns single-file import APIs, executable-bit detection and concurrent file batches. `import_file_mode/` validates sources, selects streaming or buffered input, repairs corrupt content-addressed entries from verified staging, and uses shared platform atomic replacement. Durable versus rebuildable sync flags and regular-file/symlink gates remain in force. Errors use the shared StoreError boundary.

Phụ trách API import file đơn, nhận diện executable bit và batch file song song. `import_file_mode/` kiểm source, chọn stream/buffer, sửa entry content-addressed hỏng từ staging đã xác minh và dùng atomic replacement chung của platform. Giữ cổng file thường/symlink và cờ sync durable/rebuildable. Lỗi dùng StoreError chung.

Nearest source regression tests: `test/import_file.rs` covers regular and streamed imports, including recovery after corruption. The test source is authored but NOT_RUN.
