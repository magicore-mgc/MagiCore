# Resolver cache / Cache resolver

This folder owns the in-memory registry cache used by dependency resolution. `cache.rs` and `registry_cache/mod.rs` are wiring/export surfaces; entry identity, freshness reads, bounded admission and the public cache lifecycle have separate owners.

Thư mục này sở hữu cache registry trong bộ nhớ dùng khi resolve dependency. `cache.rs` và `registry_cache/mod.rs` chỉ nối module/export; danh tính entry, đọc theo freshness, ghi có giới hạn và vòng đời API được tách owner riêng.

Each of the version and dependency maps has its own entry-count cap. The default is 2,048 entries per map; callers can set TTL and capacity with `RegistryCache::with_ttl_and_capacity`. A zero capacity disables retention. The cap bounds entry count, not the bytes held by an individual registry response.

Map version và dependency có giới hạn số entry độc lập. Mặc định là 2.048 entry mỗi map; caller đặt TTL/capacity bằng `RegistryCache::with_ttl_and_capacity`. Capacity 0 tắt lưu. Giới hạn này chặn số lượng entry, không giới hạn số byte của từng response registry.

The entry cap applies per `RegistryCache` instance and independently to both maps. Cloned cache instances retain independent maps and their own caps; process memory can therefore grow with the number of live instances. Evicting the oldest entry scans at most the configured map size under that map’s insertion gate, so bounded memory trades some write cost at capacity.

This is process-local resolver state. The Web metadata LRU and the shared on-disk package cache are separate owners and are not modified by this module. No I/O occurs here, so the cache API has no local error type.

Đây là state resolver trong process. LRU metadata Web và package cache trên đĩa là owner riêng, không bị sửa bởi module này. Module không làm I/O nên API cache không có error type riêng.
