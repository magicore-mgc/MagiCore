# Shared cache state

This owner keeps the public SharedWebCache handle and controls root discovery, writable-path probing, PackageCache setup, and once-per-process pruning. Prune policy and file removal stay in cache_prune.

Owner này giữ handle public SharedWebCache và quản lý phát hiện root, kiểm tra đường dẫn ghi được, khởi tạo PackageCache và prune một lần mỗi process. Chính sách prune và xóa file vẫn ở cache_prune.
