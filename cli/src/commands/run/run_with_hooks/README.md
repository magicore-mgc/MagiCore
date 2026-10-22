# run_with_hooks

Owns pre/main/post lifecycle ordering. A failed pre-hook aborts the chain; post runs after the main attempt; a main error retains precedence when post also fails, while the post failure is warned.

# run_with_hooks (VI)

Folder này sở hữu thứ tự lifecycle pre/main/post. Pre-hook hỏng thì dừng; post chạy sau lần thử main; nếu cả main và post cùng lỗi thì giữ lỗi main và cảnh báo lỗi post.
