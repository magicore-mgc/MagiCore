# Search cache errors / Lỗi cache tìm kiếm

SearchCacheError classifies I/O, clock, JSON serialization, and SQLite failures inside the cache owner.
SearchCacheError phân loại lỗi I/O, đồng hồ, serialize JSON và SQLite bên trong owner cache.

The public API keeps anyhow::Result and converts each variant back to its underlying error, preserving downcast and source behavior. No retry policy is added because this owner does not retry.
API công khai giữ anyhow::Result và chuyển từng variant về lỗi nền để giữ downcast cùng source. Owner này không thêm retry vì không có retry hiện hữu.
