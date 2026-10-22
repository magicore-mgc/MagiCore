# Manifest File freshness owner

Owns manifest reads, required-lock behavior and callback-based projection checking.

Sở hữu đọc manifest, quy tắc bắt buộc lock và kiểm tra projection qua callback.

The owner preserves current lock, error and ecosystem semantics; its facade only wires modules.

Owner giữ nguyên semantics lock, lỗi và ecosystem hiện tại; facade chỉ nối module.
