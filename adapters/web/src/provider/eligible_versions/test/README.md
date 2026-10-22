# Builds the resolver candidate list and applies minimum-release-age. Missing timestamps are rejected unless the explicit policy allows them; if a non-empty registry has no candidate left, it returns the existing diagnostic. tests / Test Tạo danh sách candidate cho resolver và áp dụng minimum-release-age. Timestamp thiếu bị loại trừ trừ khi policy cho phép; nếu registry có version nhưng lọc hết, trả diagnostic hiện có.

Focused cases: Fail-closed error when an active policy filters every version from a non-empty registry.

Case tập trung: Lỗi fail-closed khi policy lọc hết version trong registry không rỗng.

`mod.rs` registers `eligible_versions.rs` beside the implementation. These source tests are authored/relocated; execution remains deferred until the repository source pass and SSD gate are complete.

`mod.rs` đăng ký `eligible_versions.rs` cạnh implementation. Test đã được viết/chuyển về; chỉ chạy sau khi kết thúc source pass toàn repo và đạt giới hạn SSD.
