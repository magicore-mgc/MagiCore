# Dependency solving and metadata acquisition / Giải dependency và lấy metadata

Runs the shared resolver's root-aware solve method, preserves exact direct-root identities in `ResolutionSnapshot`, and prefetches registry metadata for every materialized resolution. This prevents supply-chain downgrade policy from inferring a direct pin from a same-name transitive version.

Gọi resolver dùng chung ở chế độ giữ identity root trực tiếp, chuyển metadata đó vào `ResolutionSnapshot`, rồi tải trước metadata registry cho các resolution. Điều này giúp policy downgrade so sánh đúng pin root, không suy đoán từ version transitive trùng tên.

Existing behavior tests stay in `adapters/web/src/test/` during this source-first pass. Tests and execution are deferred to the verification phase; authored S531/S532 nearest tests remain `NOT_RUN`.
