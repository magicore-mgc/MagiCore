# Cache entry / Entry cache

Owns an immutable value snapshot and its fetch timestamp. The timestamp defines TTL freshness and insertion age for capacity eviction; the value is shared internally and copied only when the existing public API returns an owned `Vec<T>`.

Sở hữu snapshot giá trị bất biến và thời điểm tải. Timestamp xác định TTL và tuổi insert khi eviction; giá trị được chia sẻ bên trong và chỉ copy khi API cũ trả `Vec<T>` sở hữu.
