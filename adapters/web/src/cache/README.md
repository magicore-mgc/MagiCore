# Web shared cache

This directory preserves the public crate::cache API and separates persisted cache records, root lifecycle, graph/metadata persistence, project references, package identity, and runtime policy. Existing cache_metadata and cache_prune modules remain compatibility exports.

Thư mục này giữ API public crate::cache và tách record cache, vòng đời root, persistence graph/metadata, project reference, định danh package và chính sách runtime. Các module cache_metadata và cache_prune vẫn được export tương thích.

The refactor preserves current cache rules; it does not claim a memory/disk growth fix or runtime performance improvement. Tests and build remain pending until the whole-project source pass closes.

Refactor giữ nguyên quy tắc cache hiện tại; không tuyên bố đã xử lý tăng bộ nhớ/đĩa hay cải thiện hiệu năng runtime. Test và build chờ tới khi đóng source pass toàn dự án.
