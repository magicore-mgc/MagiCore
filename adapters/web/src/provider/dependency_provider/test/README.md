# Implements the shared resolver `DependencyProvider` trait. Simple lookups remain here; enqueue and prefetch algorithms delegate to their same-name owners. tests / Test Implement trait `DependencyProvider` dùng chung của resolver. Lookup đơn giản nằm tại đây; thuật toán enqueue/prefetch chuyển tới owner cùng tên.

Focused cases: Resolver trait lookup dispatches to the provider version cache.

Case tập trung: Trait resolver gọi đúng version cache của provider.

`mod.rs` registers `dependency_provider.rs` beside the implementation. These source tests are authored/relocated; execution remains deferred until the repository source pass and SSD gate are complete.

`mod.rs` đăng ký `dependency_provider.rs` cạnh implementation. Test đã được viết/chuyển về; chỉ chạy sau khi kết thúc source pass toàn repo và đạt giới hạn SSD.
