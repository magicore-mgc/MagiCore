# HTTP response cache / Cache response HTTP

This facade preserves mgc_http::cache::{CacheEntry,HttpCache} and separates TTL validity from the shared in-memory map.

Facade này giữ mgc_http::cache::{CacheEntry,HttpCache} và tách kiểm tra TTL khỏi map trong bộ nhớ dùng chung.

Known limits are explicit: the map is unbounded, expired rows remain resident, each hit clones the full payload, and HttpClient execution does not yet consume with_cache state. These are behavior/performance work, not claims fixed by this source split.

Giới hạn đã xác minh: map không có giới hạn, entry hết hạn vẫn nằm trong bộ nhớ, mỗi cache hit clone toàn bộ payload và execute của HttpClient chưa dùng state with_cache. Đây là gap behavior/performance, không phải vấn đề đã sửa bởi lần tách source này.
