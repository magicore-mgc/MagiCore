# Logical cache size

This owner measures regular-file logical bytes for a file or directory. It never follows symbolic links, ignores special entries and returns typed errors when directory traversal fails.

Owner này đo logical bytes của file thường trong file hoặc thư mục. Nó không đi theo symlink, bỏ entry đặc biệt và trả lỗi typed khi duyệt thư mục thất bại.

The helper is shared by `cache_size` and prune, so both report the same counting semantics. Nearest behavior cases are in `test/logical_size.rs`.

Helper được `cache_size` và prune dùng chung để cùng một cách tính. Test hành vi gần nhất nằm tại `test/logical_size.rs`.
