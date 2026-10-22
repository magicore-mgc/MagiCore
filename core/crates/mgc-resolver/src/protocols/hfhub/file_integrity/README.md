# File integrity owner

Owns the integrity decision, typed mismatch errors, and public Git blob SHA-1 helper. LFS SHA-256 remains preferred; otherwise the full Git blob header and content are hashed. Exact public helper path is preserved at protocols::git_blob_sha1.

Phụ trách quyết định toàn vẹn, lỗi mismatch có type và helper Git blob SHA-1 công khai. SHA-256 LFS vẫn được ưu tiên; nếu không thì hash cả header Git blob và nội dung. Giữ nguyên đường helper công khai protocols::git_blob_sha1.
