# Tar entry writer / Bộ ghi tar entry

Owns entry naming, opened-handle metadata, safe regular-file opening, mode normalization, bounded content reads and shrink/growth detection.

Quản lý tên entry, metadata của handle đã mở, mở file thường an toàn, chuẩn hóa mode, đọc nội dung có giới hạn và phát hiện file co/giãn.

The shared platform helper rejects final-component symlinks/reparse points. Parent components are not pinned, so parent-path replacement remains an open boundary.

Helper nền tảng dùng chung chặn symlink/reparse ở component cuối. Component thư mục cha chưa được ghim nên race thay path cha vẫn còn mở.
