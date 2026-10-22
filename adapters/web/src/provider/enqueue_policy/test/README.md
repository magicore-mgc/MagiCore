# Implements optional dependency admission: cached decisions win, known native packages are filtered to the host, required edges stay enabled, and optional edges are checked against selected version metadata. tests / Test Quyết định enqueue dependency optional: dùng quyết định cache trước, lọc package native theo host, giữ required edge, và kiểm tra optional edge bằng metadata version đã chọn.

Focused cases: Required current-host platform package is kept and foreign target is skipped.

Case tập trung: Giữ package platform của host và bỏ target foreign dù edge required.

`mod.rs` registers `enqueue_policy.rs` beside the implementation. These source tests are authored/relocated; execution remains deferred until the repository source pass and SSD gate are complete.

`mod.rs` đăng ký `enqueue_policy.rs` cạnh implementation. Test đã được viết/chuyển về; chỉ chạy sau khi kết thúc source pass toàn repo và đạt giới hạn SSD.
