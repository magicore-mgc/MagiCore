# Cache prune / Dọn cache

Preserves the existing full-prune contract while taking every lock shard before removal, so no cache reader/writer runs concurrently with the sweep.
Giữ hợp đồng prune toàn phần hiện có nhưng lấy mọi shard lock trước khi xóa để không reader/writer nào chạy đồng thời với sweep.

Symlink entries are removed as links and the packages directory must remain under the configured cache root.
Entry symlink được xóa như link và thư mục packages phải nằm trong cache root đã cấu hình.
