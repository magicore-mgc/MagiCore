# Checks one explicitly selected version (dist-tag/add-latest/update). There is no fallback to another candidate: too-young or unstamped pins return the existing error unless the explicit missing-time option is enabled. tests / Test Kiểm tra một version được chọn rõ (dist-tag/add-latest/update). Không fallback candidate khác: version quá mới hoặc thiếu timestamp trả lỗi cũ, trừ khi bật tùy chọn cho phép thiếu thời gian.

Focused cases: Old pin succeeds; young and missing-time pins reject unless the escape hatch is explicit.

Case tập trung: Pin cũ thành công; pin mới/thiếu thời gian bị chặn trừ khi bật escape hatch.

`mod.rs` registers `pinned_version.rs` beside the implementation. These source tests are authored/relocated; execution remains deferred until the repository source pass and SSD gate are complete.

`mod.rs` đăng ký `pinned_version.rs` cạnh implementation. Test đã được viết/chuyển về; chỉ chạy sau khi kết thúc source pass toàn repo và đạt giới hạn SSD.
