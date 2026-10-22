# fetch

Owns cache-first retrieval of package tarballs and preserves the public `install::fetch::*` routes. URL trust/fallback decisions live in `tarball_url/`; temporary download paths, ownership cleanup, and atomic cache publication live in `staging/`.

`fetch.rs` coordinates local cache, optional shared cache, the registry instance/credentials, integrity preparation, and the download semaphore. The `error/` owner preserves local failure sources and maps to the existing shared `MgError` categories at the compatibility boundary.

Verified cache hits use a no-follow regular-file open in the install pipeline, recheck SRI on that same handle and pass it into extraction. Replacing or removing the pathname after that verification cannot redirect the extraction read. Large registry responses use the same file-backed route; the public `TarballPayload` variants remain unchanged.

# fetch / tải tarball

Quản lý việc lấy tarball theo thứ tự ưu tiên cache và giữ các đường dẫn public `install::fetch::*`. Quyết định URL tin cậy/fallback nằm trong `tarball_url/`; đường dẫn tạm, dọn file thuộc quyền sở hữu và publish cache nguyên tử nằm trong `staging/`.

`fetch.rs` điều phối cache cục bộ, cache dùng chung tùy chọn, registry/credential, chuẩn bị integrity và semaphore tải. Owner `error/` giữ source lỗi cục bộ và chuyển về các nhóm `MgError` hiện có tại ranh giới tương thích.

Cache hit đã xác minh được mở no-follow thành file thường trong install pipeline, kiểm lại SRI trên chính handle đó rồi truyền vào extraction. Xóa/thay path sau lần xác minh này không thể chuyển hướng lượt đọc extraction. Response registry lớn cũng dùng route file; các variant public của `TarballPayload` được giữ nguyên.
