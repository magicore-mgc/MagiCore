# Deduplicates aliases by source package, loads only uncached metadata, bounds concurrent requests with `metadata_concurrency_limit`, then restores results under alias keys. / Gộp alias theo source package, chỉ tải metadata chưa cache, giới hạn request đồng thời bằng `metadata_concurrency_limit`, rồi trả kết quả theo key alias.

## Responsibility

Deduplicates aliases by source package, loads only uncached metadata, bounds concurrent requests with `metadata_concurrency_limit`, then restores results under alias keys. Registry errors propagate; the first observed fetch error remains the returned failure.

## Scope and boundary

This owner contains only this behavior. `mod.rs` wires the same-name implementation and its nearest tests. It uses shared resolver/package types and keeps the established provider API and errors.

## Phạm vi và ranh giới

Gộp alias theo source package, chỉ tải metadata chưa cache, giới hạn request đồng thời bằng `metadata_concurrency_limit`, rồi trả kết quả theo key alias. Lỗi registry được truyền lên; lỗi fetch đầu tiên nhận được tiếp tục là lỗi trả về.

Owner này chỉ chứa hành vi trên. `mod.rs` nối implementation cùng tên với test gần nhất. Owner dùng type resolver/package chung và giữ nguyên API provider cùng error contract.
