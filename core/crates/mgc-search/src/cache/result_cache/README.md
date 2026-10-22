# Search result cache / Cache kết quả tìm kiếm

This owner reads and replaces search_cache rows, preserving query key, serialized payload, timestamp, and the existing seven-day TTL behavior.
Owner này đọc và thay thế row search_cache, giữ nguyên query key, payload serialize, timestamp và TTL bảy ngày hiện hành.

Expired rows currently return a miss but are not deleted. No bound or eviction is introduced in this structural move.
Row hết hạn hiện trả cache miss nhưng không bị xóa. Lát cắt cấu trúc này không thêm giới hạn hay eviction.
