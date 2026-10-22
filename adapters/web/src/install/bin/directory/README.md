# `.bin` directory / Thư mục `.bin`

Owns creation/replacement of `node_modules/.bin` and strict stale-link pruning against the expected MGC virtual-store package keys. It intentionally preserves symlinks whose resolved target is outside `.magicore`.

Quản lý tạo/thay `node_modules/.bin` và dọn link cũ ở strict mode dựa trên key package virtual-store MGC dự kiến. Owner cố ý giữ symlink có target ngoài `.magicore`.

Failures are classified as local `BinError::Directory` and converted to the same `MgError` category/message as before. No test body lives here yet; add focused filesystem safety tests during the later test phase. Runtime command: `cargo test -p mgc-web-adapter` (not run in S397).

Lỗi được phân loại thành `BinError::Directory` rồi chuyển về đúng category/message `MgError` cũ. Hiện chưa có test tại owner; bổ sung test an toàn filesystem ở phase test sau. Lệnh runtime: `cargo test -p mgc-web-adapter` (S397 chưa chạy).
