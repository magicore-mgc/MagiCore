# HttpCache / Cache HTTP trong bộ nhớ

Owns the Arc/Mutex/HashMap store, insert/get behavior, TTL check and poisoned-lock recovery. The public route remains mgc_http::cache::HttpCache.

Quản lý store Arc/Mutex/HashMap, hành vi insert/get, kiểm tra TTL và phục hồi lock poison. Đường dẫn public giữ là mgc_http::cache::HttpCache.

No local error.rs is required: the API returns Option and recovers poisoned locks using the existing policy. Capacity limits, eviction and HttpClient execution wiring remain open.

Không cần error.rs cục bộ: API trả Option và phục hồi lock poison theo chính sách cũ. Giới hạn dung lượng, eviction và nối vào execute của HttpClient vẫn còn mở.
