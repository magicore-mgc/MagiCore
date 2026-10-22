# Export File owner

Single export owns source opening, destination validation, same-handle verification and COW/copy staging, temp cleanup and atomic publish. Verification and staging consume the same no-follow regular-file handle. macOS and Linux try descriptor-based COW; other platforms use descriptor-based copy. The stage owner keeps typed I/O errors until the export boundary maps them to the existing `StoreError`. Staged bytes are hashed before publication. Parent traversal, in-place concurrent mutation, staged-temp pathname races and final destination replacement remain separate open boundaries.

Export đơn quản lý mở nguồn, kiểm tra đích, xác minh và staging COW/copy trên cùng handle, dọn temp và publish nguyên tử. Xác minh và staging dùng cùng handle file thường mở không theo symlink cuối. macOS/Linux thử COW bằng descriptor; platform khác copy bằng descriptor. Owner stage giữ lỗi I/O có kiểu tới ranh giới export rồi mới map về `StoreError` hiện có. Bytes đã stage được hash trước publish. Traversal thư mục cha, sửa nội dung đồng thời, race pathname temp và thay đích lúc publish vẫn là ranh giới riêng chưa đóng.

Nearest source test: `test/export_file.rs`; it exercises this owner implementation directly. Runtime status: NOT_RUN.
