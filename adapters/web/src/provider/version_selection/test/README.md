# Owns stable cache-key formatting, semver filtering, stable-over-prerelease preference unless the spec explicitly requests a prerelease, and registry metadata version parsing. tests / Test Quản lý format cache key, lọc semver, ưu tiên stable hơn prerelease trừ khi spec yêu cầu prerelease, và parse version từ metadata registry.

Focused cases: Stable preference and explicit prerelease selection.

Case tập trung: Ưu tiên stable và chọn prerelease khi spec yêu cầu.

`mod.rs` registers `version_selection.rs` beside the implementation. These source tests are authored/relocated; execution remains deferred until the repository source pass and SSD gate are complete.

`mod.rs` đăng ký `version_selection.rs` cạnh implementation. Test đã được viết/chuyển về; chỉ chạy sau khi kết thúc source pass toàn repo và đạt giới hạn SSD.
