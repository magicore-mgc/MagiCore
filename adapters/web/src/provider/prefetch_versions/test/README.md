# Implements the resolver version-prefetch path: use cached version lists first, then cached metadata, then batched metadata fetch; every produced list passes through the age gate. tests / Test Triển khai đường resolver prefetch version: dùng cache version, rồi metadata cache, cuối cùng fetch metadata theo batch; mọi danh sách đều qua age gate.

Focused cases: Cached version-list branch completes without a registry request.

Case tập trung: Nhánh cache version list hoàn thành không cần request registry.

`mod.rs` registers `prefetch_versions.rs` beside the implementation. These source tests are authored/relocated; execution remains deferred until the repository source pass and SSD gate are complete.

`mod.rs` đăng ký `prefetch_versions.rs` cạnh implementation. Test đã được viết/chuyển về; chỉ chạy sau khi kết thúc source pass toàn repo và đạt giới hạn SSD.
