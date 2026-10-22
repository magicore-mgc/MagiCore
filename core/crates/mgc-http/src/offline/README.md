# HTTP offline clients / HTTP client ngoại tuyến

This owner exposes cache-only OfflineClient and online-with-fallback HybridClient through the existing offline module path. It owns the stable re-exports and the local offline error taxonomy.

Owner này cung cấp OfflineClient chỉ đọc cache và HybridClient online kèm fallback qua đường dẫn module offline hiện có. Folder này quản lý export ổn định và phân loại lỗi offline cục bộ.

Known limitation: HybridClient cannot be initialized with a cache through its public API and does not save successful online responses. Its fallback cache therefore is normally empty. The fixed TTL in OfflineClient::load_cache is also retained. These product gaps are tracked separately and are not represented as working cache behavior.

Giới hạn đã xác minh từ source: API công khai của HybridClient chưa nhận cache khởi tạo và không lưu response online thành công. Vì vậy cache fallback thường trống. TTL cố định trong OfflineClient::load_cache cũng được giữ nguyên. Các gap sản phẩm này được theo dõi riêng, không được quảng bá là cache đang hoạt động.
