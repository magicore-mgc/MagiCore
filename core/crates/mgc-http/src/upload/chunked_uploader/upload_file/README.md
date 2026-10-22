# File upload / Upload file

Owns local-file opening, full SHA-256 calculation, sequential chunk orchestration, progress output and finalization.

Quản lý mở file local, tính SHA-256 toàn file, gửi chunk tuần tự, log tiến độ và finalize.

Current implementation reads the file once for hashing and again for upload; persistent resume is not wired to UploadSession. Both are explicit behavior/performance gaps.

Hiện code đọc file một lượt để hash và đọc lượt nữa để upload; resume bền vững chưa nối với UploadSession. Đây là gap behavior/performance được ghi rõ.
