# MessagePack persistence / Lưu trữ MessagePack

Loads only a version-matched, checksum-valid index; all other decoded states fall back to SQLite rebuild. Writes preserve the existing serialization, checksum framing, temporary-file name, and rename sequence.

Chỉ nạp index đúng version và checksum; mọi trạng thái giải mã không hợp lệ khác đều fallback sang dựng lại từ SQLite. Ghi file giữ nguyên serialization, cách tính checksum, tên file tạm và thứ tự rename hiện tại.
