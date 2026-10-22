# Stale lock temps / Temp lock cũ

This owner creates collision-resistant temp names and removes a candidate only when its age exceeds the configured grace and the encoded owner process is provably dead. Unknown liveness, metadata failure, and removal failure preserve the file.

Owner này tạo tên temp hạn chế va chạm và chỉ xóa khi quá thời gian grace, đồng thời pid chủ sở hữu được chứng minh đã chết. Không rõ liveness, lỗi metadata hoặc lỗi xóa đều giữ file nguyên trạng.

Call cleanup only while holding the project writer lock. The caller supplies the grace interval; cleanup does not acquire the project lock itself.

Chỉ gọi cleanup khi đang giữ project writer lock. Caller truyền grace interval; cleanup không tự acquire project lock.
