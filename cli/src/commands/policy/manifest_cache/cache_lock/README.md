# Cache lock owner / Owner khóa cache

Acquire and validate the OS lock file that serializes signed policy cache writers. The lock file remains on a stable inode and uses platform no-follow flags where available.

Lấy và xác minh lock file để tuần tự hóa cache writer. Test gần owner kiểm tra hai file handle không đồng thời giữ exclusive lock; test chưa chạy.
