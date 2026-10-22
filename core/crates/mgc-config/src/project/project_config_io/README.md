# ProjectConfig I/O owner

EN: Owns the established `ProjectConfig::load` and `save` methods. It preserves regular-file reads, missing-file behavior, execution-default precedence, marker claiming, TOML serialization and atomic publication. Errors remain at the existing `anyhow` boundary.

VI: Sở hữu `ProjectConfig::load` và `save` hiện có. Giữ nguyên đọc regular-file, xử lý file thiếu, precedence execution default, ghi nhận marker, serialize TOML và publish atomic. Lỗi tiếp tục dùng boundary `anyhow` hiện tại.
