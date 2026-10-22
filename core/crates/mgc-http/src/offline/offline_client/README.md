# OfflineClient / HTTP client ngoại tuyến

Owns TTL-based cache reads, stale metadata warnings, offline mutation rejection, and JSON cache load/save. Public behavior remains under mgc_http::offline::OfflineClient.

Quản lý đọc cache theo TTL, cảnh báo metadata cũ, chặn thao tác ghi khi offline và nạp/lưu cache JSON. API công khai vẫn nằm tại mgc_http::offline::OfflineClient.

Tests are nearest to the owner under test/. The old three test bodies are moved here without changing their assertions.

Test đặt gần owner trong test/. Ba test cũ được chuyển đến đây và giữ nguyên assertion.
