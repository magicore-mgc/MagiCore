# CacheEntry / HTTP cache entry

Owns the public CacheEntry shape and the existing elapsed < ttl validity predicate. The public route remains mgc_http::cache::CacheEntry.

Quản lý shape công khai của CacheEntry và predicate elapsed < ttl hiện có. Đường dẫn public giữ là mgc_http::cache::CacheEntry.

No local error.rs is required: validity returns bool and has no recoverable error result.

Không cần error.rs cục bộ: phép kiểm tra validity trả bool và không có lỗi có thể phục hồi.

