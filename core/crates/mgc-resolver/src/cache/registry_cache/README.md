# Registry cache / Cache registry

Owns the public `RegistryCache` methods, constructors and per-map synchronization. `RegistryCache::new()` retains the five-minute default TTL and the 2,048-entry cap for each map. `with_ttl_and_capacity` lets an embedder set both values explicitly; a capacity of zero turns off retention.

Sở hữu method, constructor và đồng bộ theo từng map của `RegistryCache`. `RegistryCache::new()` giữ TTL mặc định năm phút và giới hạn 2.048 entry mỗi map. `with_ttl_and_capacity` cho embedding application đặt rõ hai giá trị; capacity 0 tắt lưu cache.

`Clone` continues to create independent map snapshots with independent caps. `len()` counts both maps. A hit returns the same owned vector shape as before; cache misses may now occur sooner after the configured entry count is reached. Payload size is not byte-bounded, and many live cloned instances multiply the process-level map budget.

A new-key insertion uses an O(capacity) scan to find the oldest entry once the map is full. With the default cap this scan is bounded at 2,048 entries; a later benchmark phase should measure contention and high-churn workloads.

Insert key mới dùng phép quét O(capacity) để tìm entry cũ nhất khi map đầy. Với default hiện tại, lượt quét tối đa 2.048 entry; phase benchmark sau cần đo contention và workload churn cao.

`Clone` tiếp tục tạo snapshot map độc lập. `len()` đếm cả hai map. Cache hit vẫn trả vector sở hữu như trước; cache miss có thể xảy ra sớm hơn khi chạm giới hạn cấu hình.
