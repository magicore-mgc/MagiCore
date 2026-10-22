# Archive tree signature / Chữ ký cây archive

Owns common-root removal, deterministic path sorting, canonical tree-hash updates, and checked unpacked-size accumulation after entry scanning.

Sở hữu bỏ root chung, sắp xếp path xác định, cập nhật tree hash canonical và cộng dồn unpacked size có kiểm tra sau bước quét entry.

`test/tree_signature.rs` pins the empty-tree digest contract. Shared tree framing lives in `../../tree_digest/`; keep archive and extracted-tree representations aligned.

`test/tree_signature.rs` cố định contract digest cây rỗng. Framing cây dùng chung nằm ở `../../tree_digest/`; giữ biểu diễn archive và extracted tree đồng nhất.
