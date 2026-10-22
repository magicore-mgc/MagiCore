# Cached model listing

This owner lists cache-root entries that map to model IDs. Hugging Face uses the `models--<repo>` directory convention and excludes datasets, spaces, locks, version markers and symlinks. Other registry mappings retain their existing flattened-name behavior. Results are sorted for stable CLI/API output.

Owner này liệt kê entry ở cache root có thể map thành model ID. Hugging Face dùng quy ước thư mục `models--<repo>`, loại dataset, space, lock, version marker và symlink. Registry khác giữ cách map tên phẳng hiện tại. Kết quả được sắp xếp ổn định cho CLI/API.

Listing does not verify that model contents are complete or usable; it only reports recognized root entries. A missing root is an empty cache, while filesystem read/type failures and a non-directory root are returned as errors rather than a partial success. Nearest source cases are in `test/list_cached_models.rs`; typed failure context is owned by `error/`.

Listing không xác minh nội dung model đầy đủ hay dùng được; chỉ báo entry root nhận diện được. Root chưa tồn tại là cache rỗng; lỗi đọc/loại filesystem và root không phải thư mục được trả thành lỗi thay vì báo thành công với danh sách thiếu. Test gần owner nằm tại `test/list_cached_models.rs`; context lỗi typed do `error/` quản lý.
