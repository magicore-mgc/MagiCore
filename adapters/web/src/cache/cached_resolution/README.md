# Cached resolution graphs

This owner maps graph keys to cache files, checks cache schema/registry/TTL before returning a graph, and writes the existing envelope atomically. Existing expiration and invalid-file behavior are preserved.

Owner này ánh xạ graph key sang file cache, kiểm tra schema/registry/TTL trước khi trả graph và ghi envelope theo cách atomic. Hành vi hết hạn và file lỗi hiện tại được giữ nguyên.
