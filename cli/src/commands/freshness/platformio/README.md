# Platformio freshness owner

Owns the reserved PlatformIO lib_deps projection, still unwired until native lock writing exists.

Sở hữu projection lib_deps PlatformIO đang để dành, chưa nối tới khi có ghi lock native.

The owner preserves current lock, error and ecosystem semantics; its facade only wires modules.

Owner giữ nguyên semantics lock, lỗi và ecosystem hiện tại; facade chỉ nối module.
