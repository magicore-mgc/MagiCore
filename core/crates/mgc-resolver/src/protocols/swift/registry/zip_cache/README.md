# zip_cache

EN: Owns retained Swift registry archives for resolve-to-install reuse. FIFO eviction enforces both a 128 MiB default ceiling over stored `Vec` allocation capacities and a 64-entry default ceiling. `MGC_SWIFT_REGISTRY_CACHE_MAX_BYTES` and `MGC_SWIFT_REGISTRY_CACHE_MAX_ENTRIES` may lower or raise those limits up to named hard ceilings; zero disables retention. Oversized ZIPs are not retained, and every install still verifies the registry checksum.

VI: Quản lý archive registry Swift được giữ lại để dùng giữa resolve và install. FIFO eviction áp dụng đồng thời mặc định 128 MiB tính theo capacity cấp phát của các `Vec` được giữ và 64 entry. Có thể cấu hình qua `MGC_SWIFT_REGISTRY_CACHE_MAX_BYTES` và `MGC_SWIFT_REGISTRY_CACHE_MAX_ENTRIES`, tối đa tới hard ceiling có tên; giá trị 0 tắt lưu cache. ZIP vượt giới hạn sẽ không được giữ, và mỗi lần install vẫn xác minh checksum registry.

The byte limit sums retained ZIP `Vec` capacities, not allocator/map metadata. HTTP response buffering, ZIP parsing/decompression, and the `Vec` returned by `download()` can still create transient memory peaks; S446 does not claim to bound those paths. Cache errors are local typed errors and fail closed when converted to the existing resolver error API.

Giới hạn byte cộng capacity `Vec` ZIP được giữ, không gồm metadata của allocator/map. Buffer HTTP, parse/giải nén ZIP và bản `Vec` trả về từ `download()` vẫn có thể tạo đỉnh bộ nhớ tạm; S446 không tuyên bố đã giới hạn các luồng đó. Lỗi cache có kiểu cục bộ và fail-closed khi chuyển sang API lỗi resolver hiện hành.
