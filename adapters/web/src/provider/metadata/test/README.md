# Loads package metadata with a per-package async lock, maps aliases to source package names, and mirrors version-cache entries under both requested and source names. tests / Test Tải metadata với async lock theo package, ánh xạ alias sang source package và ghi version-cache dưới cả tên yêu cầu lẫn tên source.

Focused cases: Alias-to-source identity and version-cache mirroring.

Case tập trung: Identity alias sang source và ghi version-cache tương ứng.

`mod.rs` registers `metadata.rs` beside the implementation. These source tests are authored/relocated; execution remains deferred until the repository source pass and SSD gate are complete.

`mod.rs` đăng ký `metadata.rs` cạnh implementation. Test đã được viết/chuyển về; chỉ chạy sau khi kết thúc source pass toàn repo và đạt giới hạn SSD.
