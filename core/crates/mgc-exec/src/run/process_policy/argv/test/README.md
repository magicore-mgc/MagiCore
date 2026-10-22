# argv process-policy tests

These tests pin direct-image priority, keep Node inline source opaque, and verify `adb`/`xcrun` descendant classification follows the root scope while package managers remain denied.

Test này khóa ưu tiên executable image, giữ source inline Node ở trạng thái opaque, đồng thời kiểm tra descendant `adb`/`xcrun` theo scope root và vẫn deny package manager.
