# Package path containment

Centralizes lexical and canonical checks for package files. Lexical parent traversal is rejected before filesystem lookup even when the target does not exist; existing symlinks are then canonicalized and must stay within the package root.

Tập trung kiểm tra lexical và canonical cho file package. Parent traversal bị chặn trước khi tra filesystem kể cả khi target chưa tồn tại; symlink có thật tiếp tục được canonicalize và phải nằm trong package root.
