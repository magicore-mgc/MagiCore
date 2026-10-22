# NpmDependencyProvider state, registry construction including scoped chains, and per-instance age-policy arming. Changing the age policy clears cached version and optional-enqueue decisions and updates the registry fetch mode. Registry transport and HTTP behavior stay in `native/npm_registry`. tests / Test Quản lý state `NpmDependencyProvider`, khởi tạo registry (kể cả chain có scope), và bật age policy theo từng instance. Đổi age policy sẽ xóa cache version/optional-enqueue và cập nhật fetch mode của registry. Transport/HTTP vẫn thuộc `native/npm_registry`.

Focused cases: Per-instance age policy, cache invalidation, and isolation between two providers.

Case tập trung: Age policy theo instance, xóa cache khi đổi policy và tách biệt giữa hai provider.

`mod.rs` registers `provider.rs` beside the implementation. These source tests are authored/relocated; execution remains deferred until the repository source pass and SSD gate are complete.

`mod.rs` đăng ký `provider.rs` cạnh implementation. Test đã được viết/chuyển về; chỉ chạy sau khi kết thúc source pass toàn repo và đạt giới hạn SSD.
