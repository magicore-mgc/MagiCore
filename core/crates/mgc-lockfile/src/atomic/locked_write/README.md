# Locked write / Ghi có project lock

This owner enforces the `ProjectWriteLock` type at the write boundary, cleans stale temps only while that lock is held, creates a no-follow staging file, fsyncs contents, atomically replaces the destination, then syncs the directory when it can be opened.

Owner này bắt buộc kiểu `ProjectWriteLock` ở boundary ghi, chỉ dọn temp cũ khi đã giữ lock, tạo staging file không-follow symlink, fsync dữ liệu, thay destination atomic rồi sync directory nếu mở được.

Typed local I/O failures retain their source until the public compatibility mapping to `LockfileError::WriteFailed`; displayed messages remain byte-for-byte equivalent to the previous implementation. Cleanup after a failed replace remains best-effort; earlier write/sync failures continue leaving a temp for later stale cleanup.

Lỗi I/O cục bộ giữ nguyên source tới boundary ánh xạ tương thích sang `LockfileError::WriteFailed`; nội dung hiển thị giữ nguyên. Khi replace lỗi vẫn cleanup best-effort; lỗi write/sync trước đó vẫn để temp cho lần stale cleanup sau.
