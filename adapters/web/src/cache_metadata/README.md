# Web metadata cache

This owner preserves `crate::cache_metadata::*` and the compatibility re-export through `crate::cache::*`. It separates the in-memory LRU, persisted record types, TTL/stale policy, and registry fallback coordinator.

Owner này giữ `crate::cache_metadata::*` và re-export tương thích qua `crate::cache::*`. Cấu trúc tách LRU trong bộ nhớ, kiểu record lưu trữ, TTL/chính sách stale và điều phối fallback registry.

The refactor preserves existing behavior and does not claim improved memory use, cache safety, or runtime performance. Runtime verification remains deferred to the repository-wide test phase.

Refactor giữ nguyên hành vi hiện có, không khẳng định đã giảm memory, tăng an toàn cache hay hiệu năng. Kiểm chứng runtime được hoãn tới giai đoạn test toàn repository.
