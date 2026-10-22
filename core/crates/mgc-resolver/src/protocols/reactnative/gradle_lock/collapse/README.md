# Gradle pin collapse / Gộp pin Gradle

This owner preserves first-seen coordinate order, collapses equal versions across configurations, and rejects a coordinate pinned to different versions.

Owner này giữ thứ tự tọa độ xuất hiện đầu tiên, gộp version bằng nhau giữa các configuration và từ chối tọa độ bị ghim nhiều version khác nhau.

Focused tests are in `test/collapse.rs`; shared diagnostics are owned by the parent `gradle_lock/error.rs` boundary.

Test tập trung ở `test/collapse.rs`; chẩn đoán dùng chung do boundary `gradle_lock/error.rs` cấp cha sở hữu.
