# File Identity Tests

**Purpose:** Check Unix opened-handle identity and non-Unix memo disablement.

**Mục đích:** Kiểm tra identity handle đã mở trên Unix và tắt memo ngoài Unix.

Tests consume an already-open file handle so identity cannot come from a second path lookup. Source is present; runtime execution is deferred.

Test dùng handle đã mở để identity không lấy từ lần tra path thứ hai. Đã có source; runtime chờ lượt xác minh sau.
