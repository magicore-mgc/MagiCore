# Web cache pruning

This module preserves `crate::cache_prune::*` while separating prune policy/stamps, project cache cleanup, stale entries, quota selection, and filesystem helpers. The parent `crate::cache` facade continues to re-export these items.

Module này giữ API `crate::cache_prune::*` và tách policy/stamp, dọn cache project, entry cũ, chọn quota và helper filesystem. Facade `crate::cache` tiếp tục re-export các mục này.

This structural move preserves existing best-effort cleanup and does not claim improved cache safety, error visibility, disk use, or runtime performance.

Thay đổi cấu trúc giữ nguyên cơ chế dọn best-effort hiện tại; không khẳng định đã tăng an toàn cache, hiển thị lỗi, giảm dung lượng đĩa hay tăng hiệu năng runtime.
