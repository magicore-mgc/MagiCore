# Parses npm registry `time` values into Unix seconds. Invalid or incomplete text returns `None`; this owner does not decide whether missing time is allowed. tests / Test Parse trường `time` của npm registry thành Unix seconds. Chuỗi lỗi/thiếu trả `None`; owner này không tự quyết định có cho thiếu timestamp hay không.

Focused cases: Valid ISO timestamp and incomplete timestamp handling.

Case tập trung: Timestamp ISO hợp lệ và xử lý timestamp thiếu thành phần.

`mod.rs` registers `npm_timestamp.rs` beside the implementation. These source tests are authored/relocated; execution remains deferred until the repository source pass and SSD gate are complete.

`mod.rs` đăng ký `npm_timestamp.rs` cạnh implementation. Test đã được viết/chuyển về; chỉ chạy sau khi kết thúc source pass toàn repo và đạt giới hạn SSD.
