# Dependency prefetch / Tải trước dependency

This owner checks the memo cache, updates hit/miss counters, fetches only uncached dependency lists, stores provider results, invokes `on_batch_resolved`, and derives unique names for the next version-prefetch wave.

Owner này kiểm tra memo cache, cập nhật hit/miss, chỉ tải dependency chưa có, lưu kết quả provider, gọi `on_batch_resolved`, rồi lấy tên package duy nhất cho wave tải version tiếp theo.

Cache poisoning and provider failures keep their existing `SolveError` messages. Cache keys, public API and provider behavior are unchanged. Runtime verification remains deferred.
