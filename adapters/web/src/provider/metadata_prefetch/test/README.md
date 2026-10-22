# Deduplicates aliases by source package, loads only uncached metadata, bounds concurrent requests with `metadata_concurrency_limit`, then restores results under alias keys. tests / Test Gộp alias theo source package, chỉ tải metadata chưa cache, giới hạn request đồng thời bằng `metadata_concurrency_limit`, rồi trả kết quả theo key alias.

Focused cases: HTTP-backed metadata request deduplication across multiple aliases.

Case tập trung: Deduplicate request metadata thật qua HTTP khi nhiều alias cùng source.

`mod.rs` registers `metadata_prefetch.rs` beside the implementation. These source tests are authored/relocated; execution remains deferred until the repository source pass and SSD gate are complete.

`mod.rs` đăng ký `metadata_prefetch.rs` cạnh implementation. Test đã được viết/chuyển về; chỉ chạy sau khi kết thúc source pass toàn repo và đạt giới hạn SSD.
